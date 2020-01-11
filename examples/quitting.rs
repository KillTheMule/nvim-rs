//! Quitting. See src/examples/quitting.rs for documentation
use nvim_rs::{create, rpc::handler::Dummy as DummyHandler};

use std::error::Error;

use futures::task::{FutureObj, Spawn, SpawnError};

use tokio::{process::Command, spawn};

const NVIMPATH: &str = "neovim/build/bin/nvim";

struct Spawner {}

impl Spawn for Spawner {
  fn spawn_obj(
    &self,
    future: FutureObj<'static, ()>,
  ) -> Result<(), SpawnError> {
    spawn(future);
    Ok(())
  }

  fn status(&self) -> Result<(), SpawnError> {
    Ok(())
  }
}

#[tokio::main]
async fn main() {
  let handler = DummyHandler::new(Spawner {});

  let (nvim, _io_handle, _child) = create::new_child_cmd(
    Command::new(NVIMPATH)
      .args(&["-u", "NONE", "--embed", "--headless"])
      .env("NVIM_LOG_FILE", "nvimlog"),
    handler,
  )
  .await
  .unwrap();

  let chan = nvim.get_api_info().await.unwrap()[0].as_i64().unwrap();
  let close = format!("call chanclose({})", chan);

  if let Err(e) = nvim.command(&close).await {
    eprintln!("Error in last command: {}", e);
    eprintln!("Caused by : {:?}", e.as_ref().source());

    if e.is_channel_closed() {
      eprintln!("Channel closed, quitting!");
    } else {
      eprintln!("Channel was not closed, no idea what happened!");
    }
  }
}
