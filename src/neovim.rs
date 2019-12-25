use std::{
  future::Future,
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
  },
};

use crate::runtime::{oneshot, spawn, AsyncRead, AsyncWrite, BufWriter, Mutex};

use crate::{
  callerror::{CallError, DecodeError, EncodeError, LoopError},
  rpc::{handler::Handler, model, model::IntoVal},
  uioptions::UiAttachOptions,
};
use rmpv::Value;

#[macro_export]
macro_rules! call_args {
    () => (Vec::new());
    ($($e:expr), +,) => (call_args![$($e),*]);
    ($($e:expr), +) => {{
        let mut vec = Vec::new();
        $(
            vec.push($e.into_val());
        )*
        vec
    }};
}

type ResponseResult = Result<Result<Value, Value>, Arc<DecodeError>>;

type Queue = Arc<Mutex<Vec<(u64, oneshot::Sender<ResponseResult>)>>>;

/// An active Neovim session.
pub struct Requester<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) writer: Arc<Mutex<BufWriter<W>>>,
  pub(crate) queue: Queue,
  pub(crate) msgid_counter: Arc<AtomicU64>,
}

impl<W> Clone for Requester<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  fn clone(&self) -> Self {
    Requester {
      writer: self.writer.clone(),
      queue: self.queue.clone(),
      msgid_counter: self.msgid_counter.clone(),
    }
  }
}

impl<W> Requester<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub fn new<H, R>(
    reader: R,
    writer: W,
    handler: H,
  ) -> (
    Requester<<H as Handler>::Writer>,
    impl Future<Output = Result<(), Box<LoopError>>>,
  )
  where
    R: AsyncRead + Send + Unpin + 'static,
    H: Handler<Writer = W> + Send + 'static,
  {
    let req = Requester {
      writer: Arc::new(Mutex::new(BufWriter::new(writer))),
      msgid_counter: Arc::new(AtomicU64::new(0)),
      queue: Arc::new(Mutex::new(Vec::new())),
    };

    let req_t = req.clone();
    let fut = Self::io_loop(handler, reader, req_t);

    (req, fut)
  }

  async fn send_msg(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> Result<oneshot::Receiver<ResponseResult>, Box<EncodeError>> {
    let msgid = self.msgid_counter.fetch_add(1, Ordering::SeqCst);

    let req = model::RpcMessage::RpcRequest {
      msgid,
      method: method.to_owned(),
      params: args,
    };

    let (sender, receiver) = oneshot::channel();

    self.queue.lock().await.push((msgid, sender));

    let writer = self.writer.clone();
    model::encode(writer, req).await?;

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

  async fn send_error_to_callers(
    &self,
    queue: &Queue,
    err: DecodeError,
  ) -> Result<Arc<DecodeError>, Box<LoopError>> {
    let err = Arc::new(err);
    let mut v: Vec<u64> = vec![];

    let mut queue = queue.lock().await;
    queue.drain(0..).for_each(|sender| {
      let msgid = sender.0;
      sender
        .1
        .send(Err(err.clone()))
        .unwrap_or_else(|_| v.push(msgid));
    });

    if v.is_empty() {
      Ok(err)
    } else {
      Err((err, v))?
    }
  }

  async fn io_loop<H, R>(
    handler: H,
    mut reader: R,
    req: Requester<H::Writer>,
  ) -> Result<(), Box<LoopError>>
  where
    H: Handler + Sync + 'static, // TODO: Check bounds on the handler
    R: AsyncRead + Send + Unpin + 'static,
    H::Writer: AsyncWrite + Send + Sync + Unpin + 'static,
  {
    let handler = Arc::new(handler);
    let mut rest: Vec<u8> = vec![];

    loop {
      let msg = match model::decode(&mut reader, &mut rest).await {
        Ok(msg) => msg,
        Err(err) => {
          let e = req.send_error_to_callers(&req.queue, *err).await?;
          return Err(Box::new(LoopError::DecodeError(e, None)));
        }
      };

      debug!("Get message {:?}", msg);
      match msg {
        model::RpcMessage::RpcRequest {
          msgid,
          method,
          params,
        } => {
          let req = req.clone();
          let handler = handler.clone();
          spawn(async move {
            let req_t = req.clone();
            let response =
              match handler.handle_request(method, params, req_t).await {
                Ok(result) => {
                  let r = model::RpcMessage::RpcResponse {
                    msgid,
                    result,
                    error: Value::Nil,
                  };
                  r
                }
                Err(error) => model::RpcMessage::RpcResponse {
                  msgid,
                  result: Value::Nil,
                  error,
                },
              };

            let w = req.writer;
            model::encode(w, response).await.unwrap_or_else(|e| {
              error!("Error sending response to request {}: '{}'", msgid, e);
            });
          });
        }
        model::RpcMessage::RpcResponse {
          msgid,
          result,
          error,
        } => {
          let sender = find_sender(&req.queue, msgid).await?;
          if error != Value::Nil {
            sender
              .send(Ok(Err(error)))
              .map_err(|r| (msgid, r.expect("This was an OK(_)")))?;
          } else {
            sender
              .send(Ok(Ok(result)))
              .map_err(|r| (msgid, r.expect("This was an OK(_)")))?;
          }
        }
        model::RpcMessage::RpcNotification { method, params } => {
          let handler = handler.clone();
          let req = req.clone();
          spawn(
            async move { handler.handle_notify(method, params, req).await },
          );
        }
      };
    }
  }

  /// Register as a remote UI.
  ///
  /// After this method is called, the client will receive redraw notifications.
  pub async fn ui_attach(
    &mut self,
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
  pub async fn quit_no_save(&mut self) -> Result<(), Box<CallError>> {
    self.command("qa!").await
  }

}

/* The idea to use Vec here instead of HashMap
 * is that Vec is faster on small queue sizes
 * in most cases Vec.len = 1 so we just take first item in iteration.
 */
async fn find_sender(
  queue: &Queue,
  msgid: u64,
) -> Result<oneshot::Sender<ResponseResult>, Box<LoopError>> {
  let mut queue = queue.lock().await;

  let pos = match queue.iter().position(|req| req.0 == msgid) {
    Some(p) => p,
    None => return Err(msgid)?,
  };
  Ok(queue.remove(pos).1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_find_sender() {
    let queue = Arc::new(Mutex::new(Vec::new()));

    {
      let (sender, _receiver) = oneshot::channel();
      queue.lock().await.push((1, sender));
    }
    {
      let (sender, _receiver) = oneshot::channel();
      queue.lock().await.push((2, sender));
    }
    {
      let (sender, _receiver) = oneshot::channel();
      queue.lock().await.push((3, sender));
    }

    find_sender(&queue, 1).await.unwrap();
    assert_eq!(2, queue.lock().await.len());
    find_sender(&queue, 2).await.unwrap();
    assert_eq!(1, queue.lock().await.len());
    find_sender(&queue, 3).await.unwrap();
    assert!(queue.lock().await.is_empty());

    if let LoopError::MsgidNotFound(17) =
      *find_sender(&queue, 17).await.unwrap_err()
    {
    } else {
      panic!()
    }
  }
}
