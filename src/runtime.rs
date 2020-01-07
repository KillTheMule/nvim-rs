//! Re-exports of runtime functionality for async
//!
//! This tries to somehow encapsulate what we're using from [`tokio`](tokio).
//! Maybe this is of use when we try to port this to async-std or another
//! runtime, or even make it generic over the runtime.
pub use futures;

pub use futures::task::{Spawn, SpawnExt};

pub use futures::lock::Mutex;

pub use futures::io::{
  AsyncWrite, AsyncRead, AsyncWriteExt, AsyncReadExt, BufWriter, BufReader,
};
pub mod oneshot {
  pub use futures::channel::oneshot::channel;
  pub use futures::channel::oneshot::{Receiver, Sender};

  pub mod error {
    pub use futures::channel::oneshot::Canceled as RecvError;
  }
}

#[cfg(use_tokio)]
pub use tokio::{
  io::{
    stdin, stdout,
    Result, Stdin, Stdout,
  },
  net::{TcpStream, ToSocketAddrs},
  process::{Child, ChildStdin, ChildStdout, Command},
  spawn,
  task::JoinHandle,
};

#[cfg(all(unix, use_tokio))]
pub use tokio::net::UnixStream;
