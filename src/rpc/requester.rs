use std::{
  future::Future,
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
  },
};

use crate::runtime::{oneshot, spawn, AsyncRead, AsyncWrite, BufWriter, Mutex};

use crate::{
  callerror::{DecodeError, EncodeError, CallError2},
  rpc::{handler::Handler, model},
};
use rmpv::Value;

type Queue = Arc<Mutex<Vec<(u64, oneshot::Sender<Result<Value, Value>>)>>>;

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
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub fn new<H, R>(
    reader: R,
    writer: H::Writer,
    handler: H,
  ) -> (Requester<<H as Handler>::Writer>, impl Future<Output = ()>)
  where
    R: AsyncRead + Send + Unpin + 'static,
    H: Handler + Send + 'static,
    H::Writer: AsyncWrite + Send + Unpin + 'static,
  {
    let req = Requester {
      writer: Arc::new(Mutex::new(BufWriter::new(writer))),
      msgid_counter: Arc::new(AtomicU64::new(0)),
      queue: Arc::new(Mutex::new(Vec::new())),
    };

    let req_t = req.clone();

    //let dispatch_guard =
    // thread::spawn(move || Self::io_loop(handler, reader, req_t));
    let fut = Self::io_loop(handler, reader, req_t);

    (req, fut)
  }

  async fn send_msg(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> Result<oneshot::Receiver<Result<Value, Value>>, Box<EncodeError>> {
    let msgid = self.msgid_counter.fetch_add(1, Ordering::SeqCst);

    let req = model::RpcMessage::RpcRequest {
      msgid,
      method: method.to_owned(),
      params: args,
    };

    let (sender, receiver) = oneshot::channel();

    self.queue.lock().await.push((msgid, sender));

    let writer = self.writer.clone(); //&mut *self.writer.lock().unwrap();
    model::encode(writer, req)
      .await?;

    Ok(receiver)
  }

  pub async fn call(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> Result<Result<Value, Value>, Box<CallError2>> {
    let receiver = self.send_msg(method, args).await.map_err(|e|
      CallError2::SendError(*e, method.to_string()))?;

    match receiver.await {
      Ok(res) => Ok(res), // Ok(Result<Value, Value>)
      Err(err) => {
        Err(Box::new(CallError2::ReceiveError(err, method.to_string())))
      }
    }
    //receiver.await.map_err(|e| CallError2::ReceiveError(e, method.to_string()))?
  }

  async fn send_error_to_callers(&self, queue: &Queue, err: &DecodeError) {
    let mut queue = queue.lock().await;
    queue.drain(0..).for_each(|sender| {
      let e = format!("Error read response: {}", err);
      sender.1.send(Err(Value::from(e))).unwrap();
    });
  }

  async fn io_loop<H, R>(handler: H, mut reader: R, req: Requester<H::Writer>)
  where
    H: Handler + Sync + 'static,
    R: AsyncRead + Send + Unpin + 'static,
    H::Writer: AsyncWrite + Send + Sync + Unpin + 'static,
  {
    let handler = Arc::new(handler);
    let mut rest: Vec<u8> = vec![];

    loop {
      let msg = match model::decode(&mut reader, &mut rest).await {
        Ok(msg) => msg,
        Err(err) => {
          error!("Error while reading: {}", err);
          req.send_error_to_callers(&req.queue, &err).await;
          return;
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
            model::encode(w, response).await.unwrap(); //.expect("Error sending message");
          });
        }
        model::RpcMessage::RpcResponse {
          msgid,
          result,
          error,
        } => {
          let sender = find_sender(&req.queue, msgid).await;
          if error != Value::Nil {
            sender.send(Err(error)).unwrap();
          } else {
            sender.send(Ok(result)).unwrap();
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
}

/* The idea to use Vec here instead of HashMap
 * is that Vec is faster on small queue sizes
 * in most cases Vec.len = 1 so we just take first item in iteration.
 */
async fn find_sender(
  queue: &Queue,
  msgid: u64,
) -> oneshot::Sender<Result<Value, Value>> {
  let mut queue = queue.lock().await;

  let pos = queue.iter().position(|req| req.0 == msgid).unwrap();
  queue.remove(pos).1
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

    find_sender(&queue, 1).await;
    assert_eq!(2, queue.lock().await.len());
    find_sender(&queue, 2).await;
    assert_eq!(1, queue.lock().await.len());
    find_sender(&queue, 3).await;
    assert!(queue.lock().await.is_empty());
  }
}
