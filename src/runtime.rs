//! Re-exports of runtime functionality for async
//!
//! This tries to somehow encapsulate what we're using from [`tokio`](tokio).
//! Maybe this is of use when we try to port this to async-std or another
//! runtime, or even make it generic over the runtime.

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
  task::JoinHandle,
};
