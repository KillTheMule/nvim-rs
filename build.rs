use std::{
  env, path::PathBuf,
};

fn main() {
  println!(
    "cargo:rustc-env=EXAMPLES_PATH={}",
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
    .join("target/debug/examples")
    .to_str()
    .unwrap()
  );
}
