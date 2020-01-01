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
    spawn, stdin, stdout, Child, ChildStdin, Command, JoinHandle, Stdout,
    TcpStream,
  },
  Handler,
};

#[cfg(unix)]
use crate::runtime::UnixStream;

/// Connect to nvim instance via tcp
pub async fn new_tcp<H>(
  host: &str,
  port: u16,
  handler: H,
) -> io::Result<(Neovim<TcpStream>, JoinHandle<Result<(), Box<LoopError>>>)>
where
  H: Handler<Writer = TcpStream> + Send + 'static,
{
  let stream = TcpStream::connect((host, port)).await?;
  let read = TcpStream::connect((host, port)).await?;
  let (neovim, io) = Neovim::<TcpStream>::new(stream, read, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

#[cfg(unix)]
/// Connect to nvim instance via unix socket
pub async fn new_unix_socket<H, P: AsRef<Path> + Clone>(
  path: P,
  handler: H,
) -> io::Result<(Neovim<UnixStream>, JoinHandle<Result<(), Box<LoopError>>>)>
where
  H: Handler<Writer = UnixStream> + Send + 'static,
{
  let stream = UnixStream::connect(path.clone()).await?;
  let read = UnixStream::connect(path).await?;

  let (neovim, io) = Neovim::<UnixStream>::new(stream, read, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to a Neovim instance by spawning a new one.
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

/// Connect to a Neovim instance by spawning a new one
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

/// Connect to a Neovim instance by spawning a new one
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

/// Connect to a Neovim instance that spawned this process over stdin/stdout.
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
