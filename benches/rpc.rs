use criterion::{criterion_group, criterion_main, Criterion};

use futures::task::{FutureObj, Spawn, SpawnError};

use nvim_rs::{
  call_args, create::tokio as create,
  rpc::{handler::Dummy, IntoVal},
};

use tokio::{process::Command, runtime::Builder, spawn};

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

fn simple_requests(c: &mut Criterion) {
  let handler = Dummy::new(Spawner {});

  let mut rt = Builder::new()
    .threaded_scheduler()
    .enable_io()
    .build()
    .unwrap();

  let (nvim, _io_handler, _child) = rt
    .block_on(create::new_child_cmd(
      Command::new(NVIMPATH).args(&["-u", "NONE", "--embed", "--headless"]),
      handler,
    ))
    .unwrap();

  let nvim1 = nvim.clone();
  rt.block_on(async move { nvim1.command("set noswapfile").await })
    .expect("0");

  c.bench_function("simple_requests", move |b| {
    b.iter(|| {
      let nvim = nvim.clone();
      let _curbuf = rt.block_on(async move {
        nvim.get_current_buf().await.expect("1");
      });
    })
  });
}

fn request_file(c: &mut Criterion) {
  let handler = Dummy::new(Spawner {});

  let mut rt = Builder::new()
    .threaded_scheduler()
    .enable_io()
    .build()
    .unwrap();

  let (nvim, _io_handler, _child) = rt
    .block_on(create::new_child_cmd(
      Command::new(NVIMPATH).args(&[
        "-u",
        "NONE",
        "--embed",
        "--headless",
        "Cargo.lock",
      ]),
      handler,
    ))
    .unwrap();

  let nvim1 = nvim.clone();
  rt.block_on(async move { nvim1.command("set noswapfile").await })
    .expect("0");

  c.bench_function("request_file", move |b| {
    b.iter(|| {
      let nvim = nvim.clone();
      let _lines = rt.block_on(async move {
        // Using `call` is not recommended. It returns a
        // Result<Result<Value, Value, CallError>> that needs to be massaged
        // in a proper Result<Value, CallError> at least. That's what the API
        // is for, but for now we don't want to deal with getting a buffer
        // from the API
        let _ = nvim
          .call("nvim_buf_get_lines", call_args![0i64, 0i64, -1i64, false])
          .await
          .expect("1");
      });
    })
  });
}

criterion_group!(name = requests; config = Criterion::default().without_plots(); targets = simple_requests, request_file);
criterion_main!(requests);
