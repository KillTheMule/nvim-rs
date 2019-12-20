use std::{clone::Clone, result};

use crate::{
  callerror::{CallError},
  rpc::{model::IntoVal, Requester},
  uioptions::UiAttachOptions,
};

use rmpv::Value;

use crate::runtime::{AsyncWrite, Child};

/// An active Neovim session.
pub enum Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  Child(Requester<W>, Child),
  Parent(Requester<W>),
  Tcp(Requester<W>),

  #[cfg(unix)]
  UnixSocket(Requester<W>),
}

#[macro_export]
macro_rules! call_args {
    () => (Vec::new());
    ($($e:expr), +,) => (call_args![$($e),*]);
    ($($e:expr), +) => {{
        let mut vec = Vec::new();
        $(
            vec.push($e.into_val());
        )*
        vec
    }};
}

impl<W> Neovim<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub fn requester(&self) -> Requester<W> {
    use Neovim::*;

    match self {
      Child(r, _) | Parent(r) | Tcp(r) => r.clone(),
      #[cfg(unix)]
      UnixSocket(r) => r.clone(),
    }
  }

  /*
  pub fn join_dispatch_guard(self) -> thread::Result<()> {
    use Neovim::*;

    match self {
      Child(_, j, _) | Parent(_, j) | Tcp(_, j) => j.join(),
      #[cfg(unix)]
      UnixSocket(_, j) => j.join(),
    }
  }
  */

  /// Call can be made only after event loop begin processing
  pub async fn call(
    &self,
    method: &str,
    args: Vec<Value>,
  ) -> result::Result<result::Result<Value, Value>, Box<CallError>> {
    use Neovim::*;
    match self {
      Child(r, _) | Parent(r) | Tcp(r) => r.call(method, args).await,
      #[cfg(unix)]
      UnixSocket(r) => r.call(method, args).await,
    }
  }

  /// Register as a remote UI.
  ///
  /// After this method is called, the client will receive redraw notifications.
  pub async fn ui_attach(
    &mut self,
    width: i64,
    height: i64,
    opts: &UiAttachOptions,
  ) -> Result<(), Box<CallError>> {
    self
      .call(
        "nvim_ui_attach",
        call_args!(width, height, opts.to_value_map()),
      )
      .await?
      .map(|_| Ok(()))?
  }

  /// Send a quit command to Nvim.
  /// The quit command is 'qa!' which will make Nvim quit without
  /// saving anything.
  pub async fn quit_no_save(&mut self) -> Result<(), Box<CallError>> {
    self.requester().command("qa!").await
  }
}
