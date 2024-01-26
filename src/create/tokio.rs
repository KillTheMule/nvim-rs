//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session using
//! [`tokio`](tokio)
use std::{
  future::Future,
  io::{self, Error, ErrorKind, stdout},
  path::Path,
  process::Stdio,
  fs::File,
  os::fd::AsFd,
};

use tokio::{
  io::{split, stdin, WriteHalf},
  net::{TcpStream, ToSocketAddrs},
  process::{Child, ChildStdin, Command},
  spawn,
  task::JoinHandle,
  fs::File as TokioFile,
};

use parity_tokio_ipc::{Connection, Endpoint};

use tokio_util::compat::{
  Compat, TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt,
};

use crate::{create::Spawner, error::LoopError, neovim::Neovim, Handler};

impl<H> Spawner for H
where
  H: Handler,
{
  type Handle = JoinHandle<()>;

  fn spawn<Fut>(&self, future: Fut) -> Self::Handle
  where
    Fut: Future<Output = ()> + Send + 'static,
  {
    spawn(future)
  }
}

/// Connect to a neovim instance via tcp
pub async fn new_tcp<A, H>(
  addr: A,
  handler: H,
) -> io::Result<(
  Neovim<Compat<WriteHalf<TcpStream>>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = Compat<WriteHalf<TcpStream>>>,
  A: ToSocketAddrs,
{
  let stream = TcpStream::connect(addr).await?;
  let (reader, writer) = split(stream);
  let (neovim, io) = Neovim::<Compat<WriteHalf<TcpStream>>>::new(
    reader.compat(),
    writer.compat_write(),
    handler,
  );
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to a neovim instance via unix socket (Unix) or named pipe (Windows)
pub async fn new_path<H, P: AsRef<Path> + Clone>(
  path: P,
  handler: H,
) -> io::Result<(
  Neovim<Compat<WriteHalf<Connection>>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = Compat<WriteHalf<Connection>>> + Send + 'static,
{
  let stream = Endpoint::connect(path).await?;
  let (reader, writer) = split(stream);
  let (neovim, io) = Neovim::<Compat<WriteHalf<Connection>>>::new(
    reader.compat(),
    writer.compat_write(),
    handler,
  );
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to a neovim instance by spawning a new one
pub async fn new_child<H>(
  handler: H,
) -> io::Result<(
  Neovim<Compat<ChildStdin>>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = Compat<ChildStdin>> + Send + 'static,
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
  Neovim<Compat<ChildStdin>>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = Compat<ChildStdin>> + Send + 'static,
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
  Neovim<Compat<ChildStdin>>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = Compat<ChildStdin>> + Send + 'static,
{
  let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
  let stdout = child
    .stdout
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdout"))?
    .compat();
  let stdin = child
    .stdin
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdin"))?
    .compat_write();

  let (neovim, io) = Neovim::<Compat<ChildStdin>>::new(stdout, stdin, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle, child))
}

/// Connect to the neovim instance that spawned this process over stdin/stdout
pub async fn new_parent<H>(
  handler: H,
) -> Result<(
  Neovim<Compat<tokio::fs::File>>,
  JoinHandle<Result<(), Box<LoopError>>>,
), Error>
where
  H: Handler<Writer = Compat<tokio::fs::File>>,
{
  let owned_sout_fd = stdout().as_fd().try_clone_to_owned()?;
  let file = File::from(owned_sout_fd);
  let sout = TokioFile::from_std(file);

  let (neovim, io) = Neovim::<Compat<tokio::fs::File>>::new(
    stdin().compat(),
    sout.compat(),
    handler,
  );
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}
