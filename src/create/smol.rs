//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session using
//! [`smol`](smol)
use std::{future::Future, io, fs::File, path::Path};

#[cfg(unix)]
use smol::net::unix::UnixStream;

use smol::{
  Unblock,
  net::{TcpStream, AsyncToSocketAddrs},
  spawn, Task,
};

use crate::{
  create::{unbuffered_stdout, Spawner},
  error::LoopError,
  neovim::Neovim,
  Handler,
};

impl<H> Spawner for H
where
  H: Handler,
{
  type Handle = ();

  fn spawn<Fut>(&self, future: Fut) -> Self::Handle
  where
    Fut: Future<Output = ()> + Send + 'static,
  {
    spawn(future).detach();
  }
}

/// Connect to a neovim instance via tcp
pub async fn new_tcp<A, H>(
  addr: A,
  handler: H,
) -> io::Result<(
  Neovim<TcpStream>,
  Task<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = TcpStream>,
  A: AsyncToSocketAddrs,
{
  let stream = TcpStream::connect(addr).await?;
  let (reader, writer) = (stream.clone(), stream);
  let (neovim, io) =
    Neovim::<TcpStream>::new(reader, writer, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

#[cfg(unix)]
/// Connect to a neovim instance via unix socket by path. This is currently
/// only available on Unix for smol.
pub async fn new_path<H, P: AsRef<Path> + Clone>(
  path: P,
  handler: H,
) -> io::Result<(
  Neovim<UnixStream>,
  Task<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = UnixStream> + Send + 'static,
{
  let stream = UnixStream::connect(path).await?;
  let (reader, writer) = (stream.clone(), stream);
  let (neovim, io) =
    Neovim::<UnixStream>::new(reader, writer, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to the neovim instance that spawned this process over stdin/stdout
pub async fn new_parent<H>(
  handler: H,
) -> io::Result<(Neovim<Unblock<File>>, Task<Result<(), Box<LoopError>>>)>
where
  H: Handler<Writer = Unblock<File>>,
{
  let sout = Unblock::new(unbuffered_stdout()?);
  let sin = Unblock::new(std::io::stdin());
  let (neovim, io) = Neovim::<Unblock<File>>::new(sin, sout, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rpc::model::{self, RpcMessage};
  use futures::{
    StreamExt,
    channel::{mpsc, oneshot},
    future::{Either, select},
    io::AsyncWriteExt,
    lock::Mutex,
  };
  use rmpv::Value;
  use std::{sync::Arc, time::Duration};

  #[derive(Clone)]
  struct WaitingHandler {
    started_tx: mpsc::UnboundedSender<String>,
    release_rx: Arc<Mutex<oneshot::Receiver<()>>>,
    response: Result<Value, Value>,
  }

  impl Handler for WaitingHandler {
    type Writer = TcpStream;

    async fn handle_request(
      &self,
      method: String,
      _args: Vec<Value>,
      _neovim: Neovim<Self::Writer>,
    ) -> Result<Value, Value> {
      self.started_tx.unbounded_send(method).unwrap();
      let mut release_rx = self.release_rx.lock().await;
      (&mut *release_rx).await.unwrap();
      self.response.clone()
    }
  }

  async fn within_timeout<T>(
    future: impl Future<Output = T>,
    operation: &str,
  ) -> T {
    match select(
      Box::pin(future),
      Box::pin(smol::Timer::after(Duration::from_secs(2))),
    )
    .await
    {
      Either::Left((result, _)) => result,
      Either::Right(_) => panic!("Timed out waiting for {operation}"),
    }
  }

  #[test]
  fn request_handlers_survive_dropped_spawn_handles() {
    smol::block_on(async {
      for response in [Ok(Value::from(42)), Err(Value::from("handler error"))] {
        let listener =
          smol::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let (started_tx, mut started_rx) = mpsc::unbounded();
        let (release_tx, release_rx) = oneshot::channel();
        let (nvim, io) = new_tcp(
          listener.local_addr().unwrap(),
          WaitingHandler {
            started_tx,
            release_rx: Arc::new(Mutex::new(release_rx)),
            response: response.clone(),
          },
        )
        .await
        .unwrap();
        let (mut peer, _) = listener.accept().await.unwrap();
        let mut wire = Vec::new();
        model::encode_sync(
          &mut wire,
          RpcMessage::RpcRequest {
            msgid: 7,
            method: "delayed".into(),
            params: vec![],
          },
        )
        .unwrap();
        peer.write_all(&wire).await.unwrap();

        // handler_loop drops the returned spawn handle.
        assert_eq!(
          within_timeout(started_rx.next(), "the request handler to start")
            .await
            .as_deref(),
          Some("delayed")
        );
        release_tx
          .send(())
          .expect("The request handler must not be cancelled");
        let mut rest = Vec::new();
        let reply = within_timeout(
          model::decode(&mut peer, &mut rest),
          "the RPC reply",
        )
        .await
        .unwrap();
        let (result, error) = match response {
          Ok(result) => (result, Value::Nil),
          Err(error) => (Value::Nil, error),
        };
        assert_eq!(
          reply,
          RpcMessage::RpcResponse {
            msgid: 7,
            result,
            error
          }
        );
        // Only request tasks are detached; the session remains cancellable.
        io.cancel().await;
        assert!(
          nvim
            .call("after_close", vec![])
            .await
            .unwrap_err()
            .is_channel_closed()
        );
      }
    });
  }
}
