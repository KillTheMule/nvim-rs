//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session.
//!
//! This implements various possibilities to connect to neovim, including
//! spawning an own child process. Available capabilities might depend on your
//! OS and choice of features.
//!
//! Supported features: `use_tokio` and `use_async-std`.
//!
//! **IMPORTANT**: Whatever features you use, you can still use the runtime of
//! your choice. The features just determine which submodules of `create`  are
//! available. The choice of a submodule of `create` determines what types are
//! used for IO (e.g. `tokio::io::Stdin` vs. `async-std::io::Stdin`).
#[cfg(feature = "use_tokio")]
pub mod tokio;
