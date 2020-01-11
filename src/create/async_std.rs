//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session using
//! [`async-std`](async-std)
use std::io;

#[cfg(unix)]
use async_std::os::unix::net::UnixStream;

use async_std::{
  io::{stdin, stdout, Stdout},
  net::{TcpStream, ToSocketAddrs},
  path::Path,
  task::{spawn, JoinHandle},
};

use futures::io::{AsyncReadExt, WriteHalf};

use crate::{error::LoopError, neovim::Neovim, Handler};

/// Connect to a neovim instance via tcp
pub async fn new_tcp<A, H>(
  addr: A,
  handler: H,
) -> io::Result<(
  Neovim<WriteHalf<TcpStream>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = WriteHalf<TcpStream>>,
  A: ToSocketAddrs,
{
  let stream = TcpStream::connect(addr).await?;
  let (reader, writer) = stream.split();
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
  let (reader, writer) = stream.split();
  let (neovim, io) =
    Neovim::<WriteHalf<UnixStream>>::new(reader, writer, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to the neovim instance that spawned this process over stdin/stdout
pub async fn new_parent<H>(
  handler: H,
) -> (Neovim<Stdout>, JoinHandle<Result<(), Box<LoopError>>>)
where
  H: Handler<Writer = Stdout>,
{
  let (neovim, io) = Neovim::<Stdout>::new(stdin(), stdout(), handler);
  let io_handle = spawn(io);

  (neovim, io_handle)
}
