//! Scorched earth. See src/examples/scorched_earth.rs for documentation
use std::{process::exit as std_exit, sync::Arc};

use async_trait::async_trait;

use rmpv::Value;

use nvim_rs::{
  create,
  runtime::{Mutex, Stdout},
  Handler, Requester,
};

struct Posis {
  cursor_start: Option<(u64, u64)>,
  cursor_end: Option<(u64, u64)>,
}

fn the_larger(
  first: Option<(u64, u64)>,
  second: (u64, u64),
) -> Option<(u64, u64)> {
  if first.map(|t| t >= second) == Some(true) {
    first
  } else {
    Some(second)
  }
}

fn the_smaller(
  first: Option<(u64, u64)>,
  second: (u64, u64),
) -> Option<(u64, u64)> {
  if first.map(|t| t <= second) == Some(true) {
    first
  } else {
    Some(second)
  }
}

struct NH(Arc<Mutex<Posis>>);

#[async_trait]
impl Handler for NH {
  type Writer = Stdout;

  async fn handle_notify(
    &self,
    name: String,
    args: Vec<Value>,
    req: Requester<Stdout>,
  ) {
    match name.as_ref() {
      "cursor-moved-i" => {
        let line = args[0].as_u64().unwrap();
        let column = args[1].as_u64().unwrap();

        let mut posis = &mut *(self.0).lock().await;

        posis.cursor_start = the_smaller(posis.cursor_start, (line, column));
        posis.cursor_end = the_larger(posis.cursor_end, (line, column));

        req.command(&format!(
          "syntax region ScorchedEarth start=/\\%{}l\\%{}c/ end=/\\%{}l\\%{}c/",
          posis.cursor_start.unwrap().0,
          posis.cursor_start.unwrap().1,
          posis.cursor_end.unwrap().0,
          posis.cursor_end.unwrap().1
        )).await.unwrap();
      }
      "insert-enter" => {
        let _mode = args[0].as_str().unwrap();
        let line = args[1].as_u64().unwrap();
        let column = args[2].as_u64().unwrap();

        let mut posis = &mut *(self.0).lock().await;

        posis.cursor_start = Some((line, column));
        posis.cursor_end = Some((line, column));

        req
          .command("highlight link ScorchedEarth Constant")
          .await
          .unwrap();
      }
      "insert-leave" => {
        let mut posis = &mut *(self.0).lock().await;

        posis.cursor_start = None;
        posis.cursor_end = None;
        req.command("syntax clear ScorchedEarth").await.unwrap();
      }
      "quit" => {
        std_exit(0); // This needs a solution!
      }
      _ => {}
    }
  }
}

#[tokio::main]
async fn main() {
  let p = Posis {
    cursor_start: None,
    cursor_end: None,
  };
  let handler: NH = NH(Arc::new(Mutex::new(p)));

  let (_nvim, fut) = create::new_parent(handler).unwrap();
  fut.await;
}
