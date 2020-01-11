use nvim_rs::{create, rpc::handler::Dummy as DummyHandler};

use tokio::{self, spawn};

use futures::task::{FutureObj, Spawn, SpawnError};

use std::{
  process::Command,
  thread::sleep,
  time::{Duration, Instant},
};

#[cfg(unix)]
use std::path::Path;
#[cfg(unix)]
use tempdir::TempDir;

const NVIMPATH: &str = "neovim/build/bin/nvim";
const HOST: &str = "127.0.0.1";
const PORT: u16 = 6666;

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

#[tokio::test]
async fn can_connect_via_tcp() {
  let listen = HOST.to_string() + ":" + &PORT.to_string();

  let mut child = Command::new(NVIMPATH)
    .args(&["-u", "NONE", "--headless", "--listen", &listen])
    .spawn()
    .expect("Cannot start neovim");

  // wait at most 1 second for neovim to start and create the tcp socket
  let start = Instant::now();

  let (nvim, _io_handle) = loop {
    sleep(Duration::from_millis(100));

    let handler = DummyHandler::new(Spawner{});
    if let Ok(r) = create::new_tcp(&listen, handler).await {
      break r;
    } else {
      if Duration::from_secs(1) <= start.elapsed() {
        panic!("Unable to connect to neovim via tcp at {}", listen);
      }
    }
  };

  let servername = nvim
    .get_vvar("servername")
    .await
    .expect("Error retrieving servername from neovim");

  child.kill().expect("Could not kill neovim");

  assert_eq!(&listen, servername.as_str().unwrap());
}

#[cfg(unix)]
#[tokio::test]
async fn can_connect_via_unix_socket() {
  let dir = TempDir::new("neovim-lib.test")
    .expect("Cannot create temporary directory for test.");

  let socket_path = dir.path().join("unix_socket");

  let mut child = Command::new(NVIMPATH)
    .args(&["-u", "NONE", "--headless"])
    .env("NVIM_LISTEN_ADDRESS", &socket_path)
    .spawn()
    .expect("Cannot start neovim");

  // wait at most 1 second for neovim to start and create the socket
  {
    let start = Instant::now();
    let one_second = Duration::from_secs(1);
    loop {
      sleep(Duration::from_millis(100));

      if let Ok(_) = std::fs::metadata(&socket_path) {
        break;
      }

      if one_second <= start.elapsed() {
        panic!(format!("neovim socket not found at '{:?}'", &socket_path));
      }
    }
  }

  let handler = DummyHandler::new(Spawner{});
  let (nvim, _io_handle) = create::new_unix_socket(&socket_path, handler)
    .await
    .expect(&format!(
      "Unable to connect to neovim's unix socket at {:?}",
      &socket_path
    ));

  let servername = nvim
    .get_vvar("servername")
    .await
    .expect("Error retrieving servername from neovim")
    .as_str()
    .unwrap()
    .to_string();

  child.kill().expect("Could not kill neovim");

  assert_eq!(socket_path, Path::new(&servername));
}
