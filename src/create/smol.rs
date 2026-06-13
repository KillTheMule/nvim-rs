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
  type Handle = Task<()>;

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
