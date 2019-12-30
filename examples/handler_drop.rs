//! How to handle cleanup logic with access to the handler's data. See
//! src/examples/handler_drop.rs for documentation.
use nvim_rs::{
  create,
  runtime::{ChildStdin, Command},
  Handler, Neovim, Value,
};

use async_trait::async_trait;

use std::{
  fs::File,
  io::Write,
  ops::Drop,
  sync::{Arc, Mutex},
};

const OUTPUT_FILE: &str = "handler_drop.txt";
const NVIMPATH: &str = "neovim/build/bin/nvim";

struct NeovimHandler {
  buf: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = ChildStdin;

  async fn handle_notify(
    &self,
    name: String,
    args: Vec<Value>,
    _req: Neovim<ChildStdin>,
  ) {
    match name.as_ref() {
      "nvim_buf_lines_event" => {
        // This can be made more efficient by taking ownership appropriately,
        // but we skip this in this example
        for s in args[4]
          .as_array()
          .unwrap()
          .iter()
          .map(|s| s.as_str().unwrap())
        {
          self.buf.lock().unwrap().push(s.to_owned());
        }
      }
      _ => {}
    }
  }
}

impl Drop for NeovimHandler {
  fn drop(&mut self) {
    let mut file = File::create(OUTPUT_FILE).unwrap();

    for line in self.buf.lock().unwrap().iter() {
      writeln!(file, "{}", line).unwrap();
    }
  }
}

#[tokio::main]
async fn main() {
  let handler = NeovimHandler {
    buf: Arc::new(Mutex::new(vec![])),
  };

  let (nvim, io_handle, _child) = create::new_child_cmd(
    Command::new(NVIMPATH)
      .args(&["-u", "NONE", "--embed", "--headless"])
      .env("NVIM_LOG_FILE", "nvimlog"),
    handler,
  )
  .await
  .unwrap();

  let chan = nvim.get_api_info().await.unwrap()[0].as_i64().unwrap();
  let close = format!("call chanclose({})", chan);

  let curbuf = nvim.get_current_buf().await.unwrap();
  if !curbuf.attach(false, vec![]).await.unwrap() {
    return;
  }
  curbuf
    .set_lines(0, 0, false, vec!["xyz".into(), "abc".into()])
    .await
    .unwrap();

  // The next 2 calls will return an error because the channel is closed, so we
  // need to explicitely ignore it rather than unwrap it.
  let _ = nvim.command(&close).await;
  let _ = io_handle.await;
}
