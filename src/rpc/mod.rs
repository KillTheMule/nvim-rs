//! RPC functionality for [`neovim`](crate::neovim::Neovim)
//!
//! For most plugins, the main implementation work will consist of defining and
//! implementing the [`handler`](crate::rpc::handler::Handler).
pub mod handler;
pub mod model;

pub use self::model::{FromVal, IntoVal, RpcMessage};
pub use rmpv::Value;
