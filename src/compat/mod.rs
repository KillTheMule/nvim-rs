//! This module contains compatibility wrappers and other adapter thingies to
//! bridge between various async libraries.
//!
//! `nvim-rs` mainly depends on `futures-rs`, but to actually use it one needs
//! to provide an executor, in the form that the
//! [`Handler`](crate::rpc::handler::Handler) needs to implement
//! [`Spawn`](futures::task::Spawn). To actually connect to neovim, we need a
//! library to provide types that implement
//! [`AsyncWrite`](futures::io::AsyncWrite) and
//! [`AsyncRead`](futures::io::AsyncRead). Not even tokio provides such types,
//! but comes with its own traits. Therefore the need for adapters.

#[cfg(feature = "use_tokio")]
pub mod tokio;
