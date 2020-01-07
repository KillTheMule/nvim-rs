//! Re-exports of runtime functionality for async
//!
//! This tries to somehow encapsulate what we're using from [`tokio`](tokio).
//! Maybe this is of use when we try to port this to async-std or another
//! runtime, or even make it generic over the runtime.

pub use tokio::{
  io::{
    split as streamsplit, stdin, stdout, AsyncRead, AsyncReadExt, AsyncWrite,
    AsyncWriteExt, BufReader, BufWriter, Result, Stdin, Stdout, WriteHalf,
  },
  net::{TcpStream, ToSocketAddrs},
  process::{Child, ChildStdin, ChildStdout, Command},
  spawn,
  sync::{
    oneshot, Mutex,
  },
  task::JoinHandle,
};

#[cfg(unix)]
pub use tokio::net::UnixStream;
