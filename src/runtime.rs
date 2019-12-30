//use std::future::Future;

pub use tokio::{
  io::{
    stdin, stdout, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt,
    BufReader, BufWriter, Result, Stdin, Stdout,
  },
  net::{TcpStream, UnixStream},
  process::{Child, ChildStdin, ChildStdout, Command},
  spawn,
  sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot, Mutex,
  },
};

/*
pub use async_std::sync::{Sender, Receiver, channel};
//pub use async_std::task::Builder as Runtime;
use async_std::task::JoinHandle;


pub struct Runtime {}

impl Runtime {
  pub fn new() -> Self {
    Runtime {}
  }

  pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
  where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
  {
    async_std::task::spawn(future)
  }

  pub fn block_on<F: Future>(&mut self, future: F) -> F::Output
  {
    async_std::task::block_on(future)
  }
}
*/
