#![cfg(unix)]
use nvim_rs::{
  create::gio as create, create::Spawner, neovim::Neovim, Handler,
};

use async_trait::async_trait;

use rmpv::Value;

use std::{cell::RefCell, ffi::OsStr, future::Future, rc::Rc};

use gio;
use glib::MainContext;

type Stdout = gio::OutputStreamAsyncWrite<gio::PollableOutputStream>;
const NVIMPATH: &str = "neovim/build/bin/nvim";

#[derive(Clone)]
struct NeovimHandler {
  // We can use the non-threadsafe versions here, since gio runs tasks in a
  // single thread
  froodle: Rc<RefCell<String>>,
  mc: MainContext,
}

impl Spawner for NeovimHandler {
  type Handle = ();

  fn spawn<Fut>(&self, future: Fut) -> Self::Handle
  where
    Fut: Future<Output = ()> + 'static,
  {
    self.mc.spawn_local(future)
  }
}

// We need to pass ?Send so that the resulting impl does not put a `Send` bound
// on the futures
#[async_trait(?Send)]
impl Handler for NeovimHandler {
  type Writer = Stdout;

  async fn handle_request(
    &self,
    name: String,
    args: Vec<Value>,
    neovim: Neovim<Stdout>,
  ) -> Result<Value, Value> {
    match name.as_ref() {
      "dummy" => Ok(Value::from("o")),
      "req" => {
        let v = args[0].as_str().unwrap();

        let neovim = neovim.clone();
        match v {
          "y" => {
            let mut x: String = neovim
              .get_vvar("progname")
              .await
              .unwrap()
              .as_str()
              .unwrap()
              .into();
            x.push_str(" - ");
            x.push_str(
              neovim.get_var("oogle").await.unwrap().as_str().unwrap(),
            );
            x.push_str(" - ");
            x.push_str(
              neovim
                .eval("rpcrequest(1,'dummy')")
                .await
                .unwrap()
                .as_str()
                .unwrap(),
            );
            x.push_str(" - ");
            x.push_str(
              neovim
                .eval("rpcrequest(1,'req', 'z')")
                .await
                .unwrap()
                .as_str()
                .unwrap(),
            );
            Ok(Value::from(x))
          }
          "z" => {
            let x: String = neovim
              .get_vvar("progname")
              .await
              .unwrap()
              .as_str()
              .unwrap()
              .into();
            Ok(Value::from(x))
          }
          &_ => Err(Value::from("wrong argument to req")),
        }
      }
      &_ => Err(Value::from("wrong method name for request")),
    }
  }

  async fn handle_notify(
    &self,
    name: String,
    args: Vec<Value>,
    _neovim: Neovim<Stdout>,
  ) {
    match name.as_ref() {
      "set_froodle" => {
        *self.froodle.borrow_mut() = args[0].as_str().unwrap().to_string()
      }
      _ => {}
    };
  }
}

#[test]
fn nested_requests_gio() {
  let rs = r#"exe ":fun M(timer) 
      call rpcnotify(1, 'set_froodle', rpcrequest(1, 'req', 'y'))
    endfun""#;
  let rs2 = r#"exe ":fun N(timer) 
      quit
    endfun""#;

  let froodle = Rc::new(RefCell::new(String::new()));
  let mc = MainContext::new();
  let handler = NeovimHandler {
    froodle: froodle.clone(),
    mc: mc.clone(),
  };

  let (nvim, io_handle, _child) = mc
    .block_on(create::new_child_cmd(
      &[
        &OsStr::new(NVIMPATH),
        &OsStr::new("-u"),
        &OsStr::new("NONE"),
        &OsStr::new("--embed"),
        &OsStr::new("--headless"),
        &OsStr::new("-c"),
        &OsStr::new(rs),
        &OsStr::new("-c"),
        &OsStr::new(":let timer = timer_start(500, 'M')"),
        &OsStr::new("-c"),
        &OsStr::new(rs2),
        &OsStr::new("-c"),
        &OsStr::new(":let timer = timer_start(1500, 'N')"),
      ],
      handler,
    ))
    .unwrap();

  mc.block_on(nvim.set_var("oogle", Value::from("doodle")))
    .unwrap();

  // The 2nd timer closes the channel, which will be returned as an error from
  // the io handler. We only fail the test if we got another error
  if let Err(err) = mc.block_on(io_handle) {
    if !err.is_channel_closed() {
      panic!("{}", err);
    }
  }

  assert_eq!("nvim - doodle - o - nvim", froodle.borrow().to_owned());
}
