use criterion::{Criterion, criterion_group, criterion_main};
use async_trait::async_trait;
use nvim_rs::{create, Handler, call_args, rpc::IntoVal};
use nvim_rs::runtime::{ChildStdin, Command};

use tokio::runtime::Builder;

const NVIMPATH: &str = "neovim/build/bin/nvim";

struct NH{}

#[async_trait]
impl Handler for NH {
  type Writer = ChildStdin;
}

fn simple_requests(c: &mut Criterion) {
  let handler = NH{};
  //let mut rt = Runtime::new().unwrap();

  let mut rt = Builder::new().threaded_scheduler().enable_io().build().unwrap();

  let (nvim, io) = rt.block_on(create::new_child_cmd(
    Command::new(NVIMPATH)
      .args(&[
        "-u",
        "NONE",
        "--embed",
        "--headless",
      ]),
    handler,
  ))
  .unwrap();

  rt.spawn(io);
  let req = nvim.requester(); 

  let req1 = req.clone();
  rt.block_on(async move {req1.command("set noswapfile").await}).expect("0");

  c.bench_function("simple_requests", move |b| {
    //let rt = &rt;
    b.iter(|| {
        let req = nvim.requester();
        let _curbuf = rt.block_on(async move {
          req.get_current_buf().await.expect("1");
        });
      })
    });

}

fn request_file(c: &mut Criterion) {
  let handler = NH{};
  //let mut rt = Runtime::new().unwrap();
  let mut rt = Builder::new().threaded_scheduler().enable_io().build().unwrap();

  let (nvim, io) = rt.block_on(create::new_child_cmd(
    Command::new(NVIMPATH)
      .args(&[
        "-u",
        "NONE",
        "--embed",
        "--headless",
        "Cargo.lock"
      ]),
    handler,
  ))
  .unwrap();

  rt.spawn(io);
  let req = nvim.requester(); 

  let req1 = req.clone();
  rt.block_on(async move {req1.command("set noswapfile").await}).expect("0");

  c.bench_function("request_file", move |b| {
    b.iter(|| {
        let req = nvim.requester();
        let _lines = rt.block_on(async move {
          req.call("nvim_buf_get_lines",
            call_args![0i64, 0i64, -1i64, false]).await.expect("1");
        });
      })
    });

}

criterion_group!(name = requests; config = Criterion::default().without_plots(); targets = simple_requests, request_file);
criterion_main!(requests);
