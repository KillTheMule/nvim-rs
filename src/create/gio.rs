//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session using
//! [`gio`](gio)
use std::io::{self};
use std::path::Path;

use crate::{create::Spawner, error::LoopError, neovim::Neovim, Handler};

use async_std::io::{stdin, stdout, Stdout};
use futures::future::{FutureExt, RemoteHandle};

#[cfg(unix)]
use gio::{prelude::*, Subprocess, SubprocessFlags as Flags};
#[cfg(unix)]
use std::ffi::OsStr;

#[cfg(unix)]
fn other_err(msg: &str) -> io::Error {
  io::Error::new(io::ErrorKind::Other, msg)
}

/*
use glib::translate::ToGlib;
use glib::translate::FromGlibPtrFull;

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
    reader.compat_read(),
    writer.compat_write(),
    handler,
  );
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}
*/

#[cfg(unix)]
/// Connect to a neovim instance via unix socket
pub async fn new_unix_socket<H, P: AsRef<Path> + Clone>(
  path: P,
  handler: H,
) -> io::Result<(
  Neovim<gio::OutputStreamAsyncWrite<gio::PollableOutputStream>>,
  RemoteHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = gio::OutputStreamAsyncWrite<gio::PollableOutputStream>> + Spawner<Handle = ()> + 'static,
{
  let addr = gio::UnixSocketAddress::new(path.as_ref());
  let client = gio::SocketClient::new();

  let connection = client.connect_async_future(&addr).await.map_err(|_|
    other_err(&format!("Could not connect to socket address {:?}", path.as_ref())))?;

  let writer = connection
      .get_output_stream()
      .ok_or_else(|| other_err("Could not get connection's output stream"))?
      // This only works on cfg(unix), since on windows the stdin pipe is not a
      // PollableOutputStream
      .dynamic_cast::<gio::PollableOutputStream>()
      .map_err(|_| other_err("Output stream not pollable"))?
      .into_async_write()
      .map_err(|_| other_err("Conversion to AsyncWrite failed"))?;

  let reader = connection
      .get_input_stream()
      .ok_or_else(|| other_err("Could not get connection's input stream"))?
      // This only works on cfg(unix), since on windows the stdout pipe is not a
      // PollableInputStream
      .dynamic_cast::<gio::PollableInputStream>()
      .map_err(|_| other_err("Input stream not pollable"))?
      .into_async_read()
      .map_err(|_| other_err("Conversion to AsyncRead failed"))?;

  let handler2 = handler.clone();
  let (neovim, io) =
    Neovim::<gio::OutputStreamAsyncWrite<gio::PollableOutputStream>>::new(
    reader,
    writer,
    handler,
  );
  let (fut, handle) = io.remote_handle();
  handler2.spawn(fut);

  Ok((neovim, handle))
}

/// Connect to a neovim instance by spawning a new one
#[cfg(unix)]
pub async fn new_child<H>(
  handler: H,
) -> io::Result<(
  Neovim<gio::OutputStreamAsyncWrite<gio::PollableOutputStream>>,
  RemoteHandle<Result<(), Box<LoopError>>>,
  Subprocess,
)>
where
  H: Handler<Writer = gio::OutputStreamAsyncWrite<gio::PollableOutputStream>> + Spawner<Handle = ()> + 'static,
{
  if cfg!(target_os = "windows") {
    new_child_path("nvim.exe", handler).await
  } else {
    new_child_path("nvim", handler).await
  }
}

/// Connect to a neovim instance by spawning a new one
#[cfg(unix)]
pub async fn new_child_path<H, S: AsRef<Path>>(
  program: S,
  handler: H,
) -> io::Result<(
  Neovim<gio::OutputStreamAsyncWrite<gio::PollableOutputStream>>,
  RemoteHandle<Result<(), Box<LoopError>>>,
  Subprocess,
)>
where
  H: Handler<Writer = gio::OutputStreamAsyncWrite<gio::PollableOutputStream>> + Spawner<Handle = ()> + 'static,
{
  new_child_cmd(&[program.as_ref().as_ref(), &OsStr::new("--embed")], handler).await
}

/// Connect to a neovim instance by spawning a new one
#[cfg(unix)]
pub async fn new_child_cmd<H>(
  argv: &[&OsStr],
  handler: H,
) -> io::Result<(
  Neovim<gio::OutputStreamAsyncWrite<gio::PollableOutputStream>>,
  RemoteHandle<Result<(), Box<LoopError>>>,
  Subprocess,
)>
where
  H: Handler<Writer = gio::OutputStreamAsyncWrite<gio::PollableOutputStream>>
    + Spawner<Handle = ()>,
{
  let child =
    Subprocess::newv(argv, Flags::STDOUT_PIPE | Flags::STDIN_PIPE).unwrap();
  let stdout = child
    .get_stdout_pipe()
    .ok_or_else(|| other_err("Could not get child's stdout pipe"))?
    .dynamic_cast::<gio::PollableInputStream>()
    .map_err(|_| other_err("Child's stdout pipe not pollable"))?
    .into_async_read()
    .map_err(|_| other_err("Conversion to AsyncRead failed"))?;

  let stdin = child
    .get_stdin_pipe()
    .ok_or_else(|| other_err("Could not get child's stdin pipe"))?
    .dynamic_cast::<gio::PollableOutputStream>()
    .map_err(|_| other_err("Child's stdin pipe not pollable"))?
    .into_async_write()
    .map_err(|_| other_err("Conversion to AsyncWrite failed"))?;

  let handler2 = handler.clone();
  let (neovim, io) = Neovim::<
    gio::OutputStreamAsyncWrite<gio::PollableOutputStream>,
  >::new(stdout, stdin, handler);

  let (fut, handle) = io.remote_handle();
  handler2.spawn(fut);

  Ok((neovim, handle, child))
}

/// Connect to the neovim instance that spawned this process over stdin/stdout
pub async fn new_parent<H>(
  handler: H,
) -> io::Result<(Neovim<Stdout>, RemoteHandle<Result<(), Box<LoopError>>>)>
where
  H: Handler<Writer = Stdout> + Spawner<Handle = ()>,
{
  let handler2 = handler.clone();
  // TODO: Comment from Sebastian Droege of gtk-rs:
  // "if you care enough the GIOChannel API would be your solution. there's
  // g_io_channel_win32_new_fd() and the same thing for unix"
  // since we're using the async-std types here
  let (neovim, io) = Neovim::<Stdout>::new(stdin(), stdout(), handler);

  let (fut, handle) = io.remote_handle();
  handler2.spawn(fut);

  Ok((neovim, handle))
}
