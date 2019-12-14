//! Quitting. See src/examples/quitting.rs for documentation
use async_trait::async_trait;

use nvim_rs::{
  create,
  runtime::{spawn, ChildStdin, Command},
  Handler,
};

const NVIMPATH: &str = "neovim/build/bin/nvim";
struct NeovimHandler {}

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = ChildStdin;
}

#[tokio::main]
async fn main() {
  let handler = NeovimHandler {};

  let (nvim, fut) = create::new_child_cmd(
    Command::new(NVIMPATH)
      .args(&["-u", "NONE", "--embed", "--headless"])
      .env("NVIM_LOG_FILE", "nvimlog"),
    handler,
  )
  .await
  .unwrap();

  spawn(fut);

  let chan = nvim.get_api_info().await.unwrap()[0].as_i64().unwrap();
  let close = format!("call chanclose({})", chan);

  if let Err(e) = nvim.command(&close).await {
    // Not yet implemented: Better error handling, so we can actually
    // distinguish EOF from other read errors

    //if e.kind() == ErrorKind::UnexpectedEof {
    //  eprintln!("Channel closed, quitting!");
    //} else {
    eprintln!("Error in last command: {}", e);
    //}
  }
}
