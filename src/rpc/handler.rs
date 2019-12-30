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

/// The default handler defaults to doing nothing with a notification, and
/// returning a generic error for a request. It can be used if a plugin only
/// wants to send requests to neovim and get responses, but not handle any
/// notifications or requests.
pub struct DefaultHandler<Q>
where
  Q: AsyncWrite + Send + Sync + Unpin + 'static,
{
  _q: Arc<PhantomData<Q>>,
}

impl<Q> Handler for DefaultHandler<Q>
where
  Q: AsyncWrite + Send + Sync + Unpin + 'static,
{
  type Writer = Q;
}

impl<Q> DefaultHandler<Q>
where
  Q: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub fn new() -> DefaultHandler<Q> {
    DefaultHandler {
      _q: Arc::new(PhantomData),
    }
  }
}

/*
pub struct ChannelHandler<H: Handler> {
  sender: sync::Sender<(String, Vec<Value>)>,
  request_handler: H,
}

#[async_trait]
impl<H: Handler> Handler for ChannelHandler<H> {
  async fn handle_notify(
    &self,
    name: String,
    args: Vec<Value>,
    _req: Neovim<H::Writer>,
  ) {
    self.sender.send((name.to_owned(), args)).await
  }
}

#[async_trait]
impl<H: Handler> Handler for ChannelHandler<H> {
  type Writer = H::Writer;

  async fn handle_request(
    &self,
    name: String,
    args: Vec<Value>,
    req: Neovim<<H as Handler>::Writer>,
  ) -> Result<Value, Value> {
    (&*self)
      .request_handler
      .handle_request(name, args, req)
      .await
  }
}

impl<H: Handler> ChannelHandler<H> {
  pub fn new(
    request_handler: H,
  ) -> (Self, sync::Receiver<(String, Vec<Value>)>) {
    let (sender, receiver) = sync::channel(10); //TODO: Is 10 a good number?
    (
      ChannelHandler {
        request_handler,
        sender,
      },
      receiver,
    )
  }
}
*/
