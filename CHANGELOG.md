# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to Semantic Versioning.

## [Unreleased]

## 0.2.0 - 2020-08-29

### Added
- Connecting to neovim via tcp or a unix-socket (unix only) is now supported again

- The API has been updated to reflect neovim HEAD as of commit 161cdba.

### Changed
- The crate is now based on [`futures`](https://crates.io/crates/futures)
  rather than [`tokio`](https://crates.io/crates/tokio) to allow for different
  runtimes as far as possible. The features [`use_tokio`] or [`use_async-std`]
  can be used to get support for the 2 most popular rust runtimes, and give
  access to the `create::tokio` or `create::async_std` submodules that supply
  functionality to actually connect to neovim (depending on the features
  provided by the runtime library).

- The `Handler` trait now depends on `Clone`. The library used to `Arc`-wrap
  the handler anyways, so now the user has the possibility of using types that
  are cheaper to clone.

- `CallError` has a new variant `WrongType` to indicate that a message from
  neovim contained a value of the wrong type. Previously, the lib would panic
  in this case, now the user has the choice to handle it (or, more probably,
  log it properly and quit).

- `LoopError` has an additional variant `IoSpawn` that indicates that spawning
  another task with the handler has failed.

- The trait `FromVal` has been replaced by `TryUnpack`.

- As a substitute for directly passing a runtime around, the `Handler` now
  needs to implement `nvim-rs::create::Spawner`

- The function `new_parent` to connect to a parent neovim instance is now
  `async`.

## 0.1.0 - 2020-02-01
- Initial release
