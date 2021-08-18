mod common;
use common::*;

use std::{fs, path::PathBuf, process::Command};

const TESTDIR: &str = "neovim-basic-test";
const TESTFILE: &str = "neovim-basic-test/curbuf.txt";

fn viml_escape(in_str: &str) -> String {
    in_str.replace('\\', r"\\")
}

#[test]
fn basic() {
  let c1 = format!(
    "let jobid = jobstart([\"{}\", \"{}\"], {{\"rpc\": v:true}})",
    viml_escape(PathBuf::from(env!("EXAMPLES_PATH")).join("basic").to_str().unwrap()),
    TESTFILE
  );
  let c2 = r#"sleep 100m | let pong = rpcrequest(jobid, "ping")"#;
  let c3 = format!("edit {}/pong.txt| put =pong", TESTDIR);
  let c4 = r#"wqa!"#;

  let dirpath = PathBuf::from(TESTDIR);
  let _ = fs::create_dir(dirpath);

  let status = Command::new(nvim_path())
    .args(&[
      "-u",
      "NONE",
      "--headless",
      "-c",
      &c1,
      "-c",
      c2,
      "-c",
      &c3,
      "-c",
      c4,
    ])
    .status()
    .unwrap();

  assert!(status.success());

  let pong = fs::read_to_string("neovim-basic-test/pong.txt").unwrap();
  let buf = fs::read_to_string("neovim-basic-test/curbuf.txt").unwrap();

  assert_eq!("pong", pong.trim());
  assert_eq!("Ext(0, [1])", buf.trim());

  fs::remove_dir_all("neovim-basic-test").unwrap();
}
