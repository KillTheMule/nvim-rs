//! Handling notifications and request received from neovim
//!
//! The core of a plugin is defining and implementing the
//! [`handler`](crate::rpc::handler::Handler).
use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use rmpv::Value;

use crate::{runtime::AsyncWrite, Neovim};

/// The central functionality of a plugin. The trait bounds asure that each
/// asynchronous task can receive a copy of the handler, so some state can be
/// shared.
#[async_trait]
pub trait Handler: Send + Sync {
  /// The type where we write our responses to requests. Handling of incoming
  /// requests/notifications is done on the io loop, which passes the parsed
  /// messages to the handler.
  type Writer: AsyncWrite + Send + Sync + Unpin + 'static;

  /// Handling an rpc request. The ID's of requests are handled by the
  /// [`neovim`](crate::neovim::Neovim) instance.
  async fn handle_request(
    &self,
    _name: String,
    _args: Vec<Value>,
    _neovim: Neovim<Self::Writer>,
  ) -> Result<Value, Value> {
    Err(Value::from("Not implemented"))
  }

  /// Handling an rpc notification.
  async fn handle_notify(
    &self,
    _name: String,
    _args: Vec<Value>,
    _neovim: Neovim<<Self as Handler>::Writer>,
  ) {
  }
}

/// The dummy handler defaults to doing nothing with a notification, and
/// returning a generic error for a request. It can be used if a plugin only
/// wants to send requests to neovim and get responses, but not handle any
/// notifications or requests.
#[derive(Default)]
pub struct Dummy<Q>
where
  Q: AsyncWrite + Send + Sync + Unpin + 'static,
{
  _q: Arc<PhantomData<Q>>,
}

impl<Q> Handler for Dummy<Q>
where
  Q: AsyncWrite + Send + Sync + Unpin + 'static,
{
  type Writer = Q;
}

impl<Q> Dummy<Q>
where
  Q: AsyncWrite + Send + Sync + Unpin + 'static,
{
  #[must_use]
  pub fn new() -> Dummy<Q> {
    Dummy {
      _q: Arc::new(PhantomData),
    }
  }
}
