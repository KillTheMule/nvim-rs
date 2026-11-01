//! An active neovim session.
use std::{
  future::Future,
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex as SyncMutex,
  },
};

use futures::{
  channel::{
    mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    oneshot,
  },
  future,
  io::{AsyncRead, AsyncReadExt, AsyncWrite},
  lock::Mutex,
  sink::SinkExt,
  stream::StreamExt,
  TryFutureExt,
};

use crate::{
  create::Spawner,
  error::{CallError, DecodeError, EncodeError, HandshakeError, LoopError},
  rpc::{
    handler::Handler,
    model,
    model::{IntoVal, RpcMessage},
  },
  uioptions::UiAttachOptions,
};
use rmpv::Value;

/// Pack the given arguments into a `Vec<Value>`, suitable for using it for a
/// [`call`](crate::neovim::Neovim::call) to neovim.
#[macro_export]
macro_rules! call_args {
    () => (Vec::new());
    ($($e:expr_2021), +,) => (call_args![$($e),*]);
    ($($e:expr_2021), +) => {{
        let vec = vec![
          $($e.into_val(),)*
        ];
        vec
    }};
}

type ResponseResult = Result<Result<Value, Value>, Arc<DecodeError>>;

#[derive(Default)]
struct Queue {
  requests: Vec<(u64, oneshot::Sender<ResponseResult>)>,
  closed: bool,
}

impl Queue {
  fn try_push(
    &mut self,
    request: (u64, oneshot::Sender<ResponseResult>),
  ) -> Result<(), std::io::Error> {
    if self.closed {
      return Err(std::io::Error::new(
        std::io::ErrorKind::BrokenPipe,
        "RPC connection queue is closed",
      ));
    }
    self.requests.push(request);
    Ok(())
  }
}

// Locked sections only access queue state; they never run user code or await.
type SharedQueue = Arc<SyncMutex<Queue>>;

// Never hold this synchronous lock across IO or an await point.
fn close_queue(queue: &SharedQueue, error: Arc<DecodeError>) {
  let requests = {
    let mut pending = queue.lock().expect("Queue lock must not be poisoned");
    pending.closed = true;
    std::mem::take(&mut pending.requests)
  };
  for (_, sender) in requests {
    // Cancellation of an individual caller must not hide the connection error.
    let _ = sender.send(Err(error.clone()));
  }
}

/// An active Neovim session.
pub struct Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) writer: Arc<Mutex<W>>,
  queue: SharedQueue,
  pub(crate) msgid_counter: Arc<AtomicU64>,
}

impl<W> Clone for Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  fn clone(&self) -> Self {
    Neovim {
      writer: self.writer.clone(),
      queue: self.queue.clone(),
      msgid_counter: self.msgid_counter.clone(),
    }
  }
}

impl<W> PartialEq for Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.writer, &other.writer)
  }
}
impl<W> Eq for Neovim<W> where W: AsyncWrite + Send + Unpin + 'static {}

impl<W> Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  #[allow(clippy::new_ret_no_self)]
  pub fn new<H, R>(
    reader: R,
    writer: W,
    handler: H,
  ) -> (
    Neovim<<H as Handler>::Writer>,
    impl Future<Output = Result<(), Box<LoopError>>>,
  )
  where
    R: AsyncRead + Send + Unpin + 'static,
    H: Handler<Writer = W> + Spawner,
  {
    let req = Neovim {
      writer: Arc::new(Mutex::new(writer)),
      msgid_counter: Arc::new(AtomicU64::new(0)),
      queue: Arc::new(SyncMutex::new(Queue::default())),
    };

    let (sender, receiver) = unbounded();
    let fut = future::try_join(
      req.clone().io_loop(reader, sender),
      req.clone().handler_loop(handler, receiver),
    )
    .map_ok(|_| ());

    (req, fut)
  }

  /// Create a new instance, immediately send a handshake message and
  /// wait for the response. Unlike `new`, this function is tolerant to extra
  /// data in the reader before the handshake response is received.
  ///
  /// `message` should be a unique string that is normally not found in the
  /// stdout. Due to the way Neovim packs strings, the length has to be either
  /// less than 20 characters or more than 31 characters long.
  /// See https://github.com/neovim/neovim/issues/32784 for more information.
  pub async fn handshake<H, R>(
    mut reader: R,
    writer: W,
    handler: H,
    message: &str,
  ) -> Result<
    (
      Neovim<<H as Handler>::Writer>,
      impl Future<Output = Result<(), Box<LoopError>>> + use<H, R, W>,
    ),
    Box<HandshakeError>,
  >
  where
    R: AsyncRead + Send + Unpin + 'static,
    H: Handler<Writer = W> + Spawner,
  {
    let instance = Neovim {
      writer: Arc::new(Mutex::new(writer)),
      msgid_counter: Arc::new(AtomicU64::new(0)),
      queue: Arc::new(SyncMutex::new(Queue::default())),
    };

    let msgid = instance.msgid_counter.fetch_add(1, Ordering::SeqCst);
    // Nvim encodes fixed size strings with a length of 20-31 bytes wrong, so
    // avoid that
    let msg_len = message.len();
    assert!(
      !(20..=31).contains(&msg_len),
      "The message should be less than 20 characters or more than 31 characters
      long, but the length is {msg_len}."
    );

    let req = RpcMessage::RpcRequest {
      msgid,
      method: "nvim_exec_lua".to_owned(),
      params: call_args![format!("return '{message}'"), Vec::<Value>::new()],
    };
    model::encode(instance.writer.clone(), req).await?;

    let expected_resp = RpcMessage::RpcResponse {
      msgid,
      error: rmpv::Value::Nil,
      result: rmpv::Value::String(message.into()),
    };
    let mut expected_data = Vec::new();
    model::encode_sync(&mut expected_data, expected_resp)
      .expect("Encoding static data can't fail");
    let mut actual_data = Vec::new();
    let mut start = 0;
    let mut end = 0;
    while end - start != expected_data.len() {
      actual_data.resize(start + expected_data.len(), 0);

      let bytes_read =
        reader
          .read(&mut actual_data[start..])
          .await
          .map_err(|err| {
            (
              err,
              String::from_utf8_lossy(&actual_data[..end]).to_string(),
            )
          })?;
      if bytes_read == 0 {
        // The end of the stream has been reached when the reader returns Ok(0).
        // Since we haven't detected a suitable response yet, return an error.
        return Err(Box::new(HandshakeError::UnexpectedResponse(
          String::from_utf8_lossy(&actual_data[..end]).to_string(),
        )));
      }
      end += bytes_read;
      while end - start > 0 {
        if actual_data[start..end] == expected_data[..end - start] {
          break;
        }
        start += 1;
      }
    }

    let (sender, receiver) = unbounded();
    let fut = future::try_join(
      instance.clone().io_loop(reader, sender),
      instance.clone().handler_loop(handler, receiver),
    )
    .map_ok(|_| ());

    Ok((instance, fut))
  }

  async fn send_msg(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> Result<oneshot::Receiver<ResponseResult>, Box<EncodeError>> {
    let msgid = self.msgid_counter.fetch_add(1, Ordering::SeqCst);

    let req = RpcMessage::RpcRequest {
      msgid,
      method: method.to_owned(),
      params: args,
    };

    let (sender, receiver) = oneshot::channel();

    {
      let mut pending =
        self.queue.lock().expect("Queue lock must not be poisoned");
      pending.try_push((msgid, sender))?;
    }

    let writer = self.writer.clone();
    if let Err(error) = model::encode(writer, req).await {
      // A failed write may have left a partial frame on the stream. Do not
      // allow later calls to reuse it, and release all pending callers.
      close_queue(
        &self.queue,
        Arc::new(DecodeError::ReaderError(std::io::Error::new(
          std::io::ErrorKind::BrokenPipe,
          "RPC write failed",
        ))),
      );
      return Err(error);
    }

    Ok(receiver)
  }

  pub async fn call(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> Result<Result<Value, Value>, Box<CallError>> {
    let receiver = self
      .send_msg(method, args)
      .await
      .map_err(|e| CallError::SendError(*e, method.to_string()))?;

    match receiver.await {
      // Result<Result<Result<Value, Value>, Arc<DecodeError>>, RecvError>
      Ok(Ok(r)) => Ok(r), // r is Result<Value, Value>, i.e. we got an answer
      Ok(Err(err)) => {
        // err is a Decode Error, i.e. the answer wasn't decodable
        Err(Box::new(CallError::DecodeError(err, method.to_string())))
      }
      Err(err) => {
        // err is RecvError
        Err(Box::new(CallError::InternalReceiveError(
          err,
          method.to_string(),
        )))
      }
    }
  }

  async fn handler_loop<H>(
    self,
    handler: H,
    mut receiver: UnboundedReceiver<RpcMessage>,
  ) -> Result<(), Box<LoopError>>
  where
    H: Handler<Writer = W> + Spawner,
  {
    loop {
      let msg = match receiver.next().await {
        Some(msg) => msg,
        /* If our receiver closes, that just means that io_handler started
         * shutting down. This is normal, so shut down along with it and don't
         * report an error
         */
        None => break Ok(()),
      };

      match msg {
        RpcMessage::RpcRequest {
          msgid,
          method,
          params,
        } => {
          let handler_c = handler.clone();
          let neovim = self.clone();
          let writer = self.writer.clone();

          handler.spawn(async move {
            let response =
              match handler_c.handle_request(method, params, neovim).await {
                Ok(result) => RpcMessage::RpcResponse {
                  msgid,
                  result,
                  error: Value::Nil,
                },
                Err(error) => RpcMessage::RpcResponse {
                  msgid,
                  result: Value::Nil,
                  error,
                },
              };

            model::encode(writer, response).await.unwrap_or_else(|e| {
              error!("Error sending response to request {}: '{}'", msgid, e);
            });
          });
        }
        RpcMessage::RpcNotification { method, params } => {
          handler.handle_notify(method, params, self.clone()).await
        }
        RpcMessage::RpcResponse { .. } => unreachable!(),
      }
    }
  }

  fn io_loop<R>(
    self,
    mut reader: R,
    mut sender: UnboundedSender<RpcMessage>,
  ) -> impl Future<Output = Result<(), Box<LoopError>>>
  where
    R: AsyncRead + Send + Unpin + 'static,
  {
    struct ConnectionGuard(SharedQueue);

    impl Drop for ConnectionGuard {
      fn drop(&mut self) {
        // This is a synthetic shutdown error: the RPC response producer is gone,
        // even if the transport itself is still open. BrokenPipe denotes that
        // unusable channel; it does not claim that a peer sent EOF.
        // Decode failures already release callers with their original error.
        close_queue(
          &self.0,
          Arc::new(DecodeError::ReaderError(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "RPC IO loop stopped",
          ))),
        );
      }
    }

    // An async fn would construct the guard only on its first poll.
    // Construct the guard before polling so dropping an unpolled IO future
    // also closes the session and wakes pending calls.
    let guard = ConnectionGuard(self.queue.clone());
    async move {
      let _guard = guard;
      let mut rest: Vec<u8> = vec![];

      loop {
        let msg = match model::decode(&mut reader, &mut rest).await {
          Ok(msg) => msg,
          Err(err) => {
            let e = Arc::new(*err);
            close_queue(&self.queue, e.clone());
            return Err(Box::new(LoopError::DecodeError(e, None)));
          }
        };

        debug!("Get message {:?}", msg);
        if let RpcMessage::RpcResponse {
          msgid,
          result,
          error,
        } = msg
        {
          let sender = find_sender(&self.queue, msgid)?;
          let response = if error == Value::Nil {
            Ok(result)
          } else {
            Err(error)
          };
          // A caller may have timed out after sending its request. Consume the
          // late response without disconnecting other callers or notifications.
          if sender.send(Ok(response)).is_err() {
            debug!("Discarding response to cancelled request {}", msgid);
          }
        } else {
          // Send message to handler_loop()
          sender.send(msg).await.unwrap();
        }
      } else {
        // Send message to handler_loop()
        //
        // TODO: This unwrap makes me uncomfortable. Let's extend LoopError and
        // return it here. There's not much to do after the handler_loop went
        // away (which is the only error condition here), but let's not tear
        // down stuff we don't need to.
        sender.send(msg).await.unwrap();
      }
    }
  }

  /// Register as a remote UI.
  ///
  /// After this method is called, the client will receive redraw notifications.
  pub async fn ui_attach(
    &self,
    width: i64,
    height: i64,
    opts: &UiAttachOptions,
  ) -> Result<(), Box<CallError>> {
    self
      .call(
        "nvim_ui_attach",
        call_args!(width, height, opts.to_value_map()),
      )
      .await?
      .map(|_| Ok(()))?
  }

  /// Send a quit command to Nvim.
  /// The quit command is 'qa!' which will make Nvim quit without
  /// saving anything.
  pub async fn quit_no_save(&self) -> Result<(), Box<CallError>> {
    self.command("qa!").await
  }
}

/* The idea to use Vec here instead of HashMap
 * is that Vec is faster on small queue sizes
 * in most cases Vec.len = 1 so we just take first item in iteration.
 */
fn find_sender(
  queue: &SharedQueue,
  msgid: u64,
) -> Result<oneshot::Sender<ResponseResult>, Box<LoopError>> {
  let mut pending = queue.lock().expect("Queue lock must not be poisoned");
  let queue = &mut pending.requests;

  let pos = match queue.iter().position(|req| req.0 == msgid) {
    Some(p) => p,
    None => return Err(msgid.into()),
  };
  Ok(queue.remove(pos).1)
}

#[cfg(all(test, feature = "use_tokio"))]
mod tests {
  use super::*;

  fn session<R: AsyncRead + Send + Unpin + 'static>(
    reader: R,
  ) -> (
    Neovim<futures::io::Sink>,
    impl Future<Output = Result<(), Box<LoopError>>>,
  ) {
    Neovim::new(
      reader,
      futures::io::sink(),
      crate::rpc::handler::Dummy::<futures::io::Sink>::new(),
    )
  }

  async fn assert_disconnected(
    nvim: &Neovim<futures::io::Sink>,
    mut pending: oneshot::Receiver<ResponseResult>,
  ) {
    // Shutdown has completed, so notification must already be available.
    let result = pending
      .try_recv()
      .expect("Shutdown must send an error before dropping the sender")
      .expect("Shutdown must release pending calls synchronously");
    assert!(result.is_err());
    assert!(nvim
      .queue
      .lock()
      .expect("Queue lock must not be poisoned")
      .requests
      .is_empty());
    let error = nvim
      .call("after_close", vec![])
      .await
      .expect_err("RPC shutdown must return an error");
    assert!(error.is_channel_closed());
  }

  #[tokio::test]
  async fn dropping_unpolled_io_releases_callers() {
    let (nvim, io) = session(futures::io::Cursor::new(Vec::<u8>::new()));
    let pending = nvim
      .send_msg("pending", vec![])
      .await
      .expect("Test request must be written to the sink");
    drop(io);
    assert_disconnected(&nvim, pending).await;
  }

  #[tokio::test]
  async fn aborting_io_releases_callers() {
    use tokio_util::compat::TokioAsyncReadCompatExt;
    let (reader, _peer) = tokio::io::duplex(1024);
    let (nvim, io) = session(reader.compat());
    let mut io = Box::pin(io);
    let pending = nvim
      .send_msg("pending", vec![])
      .await
      .expect("Test request must be written to the sink");
    // Ensure this tests cancellation after polling, not an unpolled future.
    assert!(futures::poll!(io.as_mut()).is_pending());
    let task = tokio::spawn(io);
    task.abort();
    assert!(task
      .await
      .expect_err("Aborted task must report cancellation")
      .is_cancelled());
    assert_disconnected(&nvim, pending).await;
  }

  #[tokio::test]
  async fn eof_with_cancelled_call_preserves_original_error() {
    let (nvim, io) = session(futures::io::Cursor::new(Vec::<u8>::new()));
    drop(
      nvim
        .send_msg("cancelled", vec![])
        .await
        .expect("Test request must be written to the sink"),
    );
    let pending = nvim
      .send_msg("pending", vec![])
      .await
      .expect("Test request must be written to the sink");
    let error = io.await.expect_err("RPC shutdown must return an error");
    assert!(matches!(*error, LoopError::DecodeError(_, None)));
    assert!(error.is_channel_closed());
    assert_disconnected(&nvim, pending).await;
  }

  #[tokio::test]
  async fn unknown_response_releases_other_callers() {
    let mut wire = Vec::new();
    model::encode_sync(
      &mut wire,
      RpcMessage::RpcResponse {
        msgid: 999,
        error: Value::Nil,
        result: Value::Nil,
      },
    )
    .expect("Test response must encode into memory");
    let (nvim, io) = session(futures::io::Cursor::new(wire));
    let pending = nvim
      .send_msg("pending", vec![])
      .await
      .expect("Test request must be written to the sink");
    assert!(matches!(
      *io.await.expect_err("RPC shutdown must return an error"),
      LoopError::MsgidNotFound(999)
    ));
    assert_disconnected(&nvim, pending).await;
  }

  #[tokio::test]
  async fn write_failure_closes_pending_requests() {
    // Opening a file read-only gives a deterministic failing writer.
    let file = tempfile::NamedTempFile::new()
      .expect("Temporary test file must be created");
    let writer = futures::io::AllowStdIo::new(
      std::fs::File::open(file.path())
        .expect("Existing test file must open read-only"),
    );
    let (nvim, _io) = Neovim::new(
      futures::io::Cursor::new(Vec::<u8>::new()),
      writer,
      crate::rpc::handler::Dummy::<futures::io::AllowStdIo<std::fs::File>>::new(
      ),
    );
    let (tx, mut rx) = oneshot::channel();
    nvim
      .queue
      .lock()
      .expect("Queue lock must not be poisoned")
      .requests
      .push((999, tx));
    assert!(nvim.call("write", vec![]).await.is_err());
    assert!(rx
      .try_recv()
      .expect("Write failure must notify pending callers")
      .expect("Failed write must release pending calls synchronously")
      .is_err());
    assert!(nvim
      .queue
      .lock()
      .expect("Queue lock must not be poisoned")
      .requests
      .is_empty());
    assert!(nvim
      .call("after_close", vec![])
      .await
      .expect_err("RPC shutdown must return an error")
      .is_channel_closed());
  }

  #[tokio::test]
  async fn late_responses_after_timeout_preserve_connection() {
    use std::time::Duration;

    for error in [Value::Nil, Value::from("late error")] {
      let nvim = Neovim {
        writer: Arc::new(Mutex::new(futures::io::sink())),
        queue: Arc::new(SyncMutex::new(Queue::default())),
        msgid_counter: Arc::new(AtomicU64::new(0)),
      };
      // Time out only after the complete request has been written.
      let cancelled = nvim.send_msg("slow", vec![]).await.unwrap();
      assert!(tokio::time::timeout(Duration::from_millis(1), cancelled)
        .await
        .is_err());
      let active = nvim.send_msg("next", vec![]).await.unwrap();
      let mut wire = Vec::new();
      for message in [
        RpcMessage::RpcResponse {
          msgid: 0,
          error,
          result: Value::Nil,
        },
        RpcMessage::RpcResponse {
          msgid: 1,
          error: Value::Nil,
          result: Value::from(42),
        },
        RpcMessage::RpcNotification {
          method: "redraw".into(),
          params: vec![],
        },
      ] {
        model::encode_sync(&mut wire, message).unwrap();
      }
      let (tx, mut rx) = unbounded();
      // EOF ends this finite stream, but both replies and the notification
      // must have been processed before the connection reports EOF.
      let end = nvim
        .clone()
        .io_loop(futures::io::Cursor::new(wire), tx)
        .await;
      assert!(matches!(*end.unwrap_err(), LoopError::DecodeError(..)));
      assert_eq!(active.await.unwrap().unwrap().unwrap(), Value::from(42));
      assert!(matches!(rx.next().await,
        Some(RpcMessage::RpcNotification { method, .. }) if method == "redraw"));
      assert!(nvim
        .queue
        .lock()
        .expect("Queue lock must not be poisoned")
        .requests
        .is_empty());
    }
  }

  #[tokio::test]
  async fn test_find_sender() {
    let queue = Arc::new(SyncMutex::new(Queue::default()));

    {
      let (sender, _receiver) = oneshot::channel();
      queue
        .lock()
        .expect("Queue lock must not be poisoned")
        .requests
        .push((1, sender));
    }
    {
      let (sender, _receiver) = oneshot::channel();
      queue
        .lock()
        .expect("Queue lock must not be poisoned")
        .requests
        .push((2, sender));
    }
    {
      let (sender, _receiver) = oneshot::channel();
      queue
        .lock()
        .expect("Queue lock must not be poisoned")
        .requests
        .push((3, sender));
    }

    find_sender(&queue, 1).unwrap();
    assert_eq!(
      2,
      queue
        .lock()
        .expect("Queue lock must not be poisoned")
        .requests
        .len()
    );
    find_sender(&queue, 2).unwrap();
    assert_eq!(
      1,
      queue
        .lock()
        .expect("Queue lock must not be poisoned")
        .requests
        .len()
    );
    find_sender(&queue, 3).unwrap();
    assert!(queue
      .lock()
      .expect("Queue lock must not be poisoned")
      .requests
      .is_empty());

    if let LoopError::MsgidNotFound(17) = *find_sender(&queue, 17).unwrap_err()
    {
    } else {
      panic!()
    }
  }
}
