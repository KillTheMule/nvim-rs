//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session.
//!
//! This implements various possibilities to connect to neovim, including
//! spawning an own child process. Available capabilities might depend on your
//! OS.
use std::{
  io::{self, Error, ErrorKind},
  path::Path,
  process::Stdio,
};

use crate::{
  error::LoopError,
  neovim::Neovim,
  runtime::{
    spawn, stdin, stdout, streamsplit, Child, ChildStdin, Command, JoinHandle,
    Stdout, TcpStream, ToSocketAddrs, WriteHalf,
  },
  Handler,
};

#[cfg(unix)]
use crate::runtime::UnixStream;

/// Connect to a neovim instance via tcp
pub async fn new_tcp<A, H>(
  addr: A,
  handler: H,
) -> io::Result<(
  Neovim<WriteHalf<TcpStream>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = WriteHalf<TcpStream>> + Send + 'static,
  A: ToSocketAddrs,
{
  let stream = TcpStream::connect(addr).await?;
  let (reader, writer) = streamsplit(stream);
  let (neovim, io) =
    Neovim::<WriteHalf<TcpStream>>::new(reader, writer, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

#[cfg(unix)]
/// Connect to a neovim instance via unix socket
pub async fn new_unix_socket<H, P: AsRef<Path> + Clone>(
  path: P,
  handler: H,
) -> io::Result<(
  Neovim<WriteHalf<UnixStream>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = WriteHalf<UnixStream>> + Send + 'static,
{
  let stream = UnixStream::connect(path).await?;
  let (reader, writer) = streamsplit(stream);
  let (neovim, io) =
    Neovim::<WriteHalf<UnixStream>>::new(reader, writer, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to a neovim instance by spawning a new one
pub async fn new_child<H>(
  handler: H,
) -> io::Result<(
  Neovim<ChildStdin>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = ChildStdin> + Send + 'static,
{
  if cfg!(target_os = "windows") {
    new_child_path("nvim.exe", handler).await
  } else {
    new_child_path("nvim", handler).await
  }
}

/// Connect to a neovim instance by spawning a new one
pub async fn new_child_path<H, S: AsRef<Path>>(
  program: S,
  handler: H,
) -> io::Result<(
  Neovim<ChildStdin>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = ChildStdin> + Send + 'static,
{
  new_child_cmd(Command::new(program.as_ref()).arg("--embed"), handler).await
}

/// Connect to a neovim instance by spawning a new one
///
/// stdin/stdout will be rewritten to `Stdio::piped()`
pub async fn new_child_cmd<H>(
  cmd: &mut Command,
  handler: H,
) -> io::Result<(
  Neovim<ChildStdin>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = ChildStdin> + Send + 'static,
{
  let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
  let stdout = child
    .stdout()
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdout"))?;
  let stdin = child
    .stdin()
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdin"))?;

  let (neovim, io) = Neovim::<ChildStdin>::new(stdout, stdin, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle, child))
}

/// Connect to the neovim instance that spawned this process over stdin/stdout
pub fn new_parent<H>(
  handler: H,
) -> io::Result<(Neovim<Stdout>, JoinHandle<Result<(), Box<LoopError>>>)>
where
  H: Handler<Writer = Stdout> + Send + 'static,
{
  let (neovim, io) = Neovim::<Stdout>::new(stdin(), stdout(), handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}
