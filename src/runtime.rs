//use std::future::Future;

pub use tokio::sync::mpsc::{Sender, Receiver, channel};
pub use tokio::sync::oneshot;
pub use tokio::sync::Mutex;
pub use tokio::spawn;
pub use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt,
BufWriter, BufReader, Stdin, Stdout, stdin, stdout, Result};
pub use tokio::net::{TcpStream, UnixStream};
pub use tokio::process::{ChildStdin, ChildStdout, Command, Child};

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
