#![cfg(unix)]
//! Quitting. See src/examples/quitting.rs for documentation
use nvim_rs::{create::gio as create, create::Spawner, rpc::handler::Handler};

use std::ffi::OsStr;
use glib::MainContext;
use gio;

use std::error::Error;
use std::future::Future;

const NVIMPATH: &str = "neovim/build/bin/nvim";

#[derive(Clone)]
struct NeovimHandler {
  mc: MainContext
}

impl Handler for NeovimHandler {
  type Writer = gio::OutputStreamAsyncWrite<gio::PollableOutputStream>;
}

impl Spawner for NeovimHandler {
  type Handle = ();

  fn spawn<Fut>(&self, future: Fut) -> Self::Handle
  where
    Fut: Future<Output = ()> + 'static
  {
    self.mc.spawn_local(future)
  }
}

fn main() {
  let mc = MainContext::new();
  let handler = NeovimHandler{ mc: mc.clone() };

  let (nvim, _handle, _child) = mc.block_on(create::new_child_cmd(
     &[
       &OsStr::new(NVIMPATH),
       &OsStr::new("-u"),
       &OsStr::new("NONE"),
       &OsStr::new("--embed"),
       &OsStr::new("--headless"),
     ],
    handler,
  ))
  .unwrap();

  let chan = mc.block_on(nvim.get_api_info()).unwrap()[0].as_i64().unwrap();
  let close = format!("call chanclose({})", chan);

  if let Err(e) = mc.block_on(nvim.command(&close)) {
    eprintln!("Error in last command: {}", e);
    eprintln!("Caused by : {:?}", e.as_ref().source());

    if e.is_channel_closed() {
      eprintln!("Channel closed, quitting!");
    } else {
      eprintln!("Channel was not closed, no idea what happened!");
    }
  }
}
