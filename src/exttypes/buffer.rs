use futures::io::AsyncWrite;
use rmpv::Value;

use crate::{rpc::model::IntoVal, Neovim, impl_exttype_traits};
/// A struct representing a neovim buffer. It is specific to a
/// [`Neovim`](crate::neovim::Neovim) instance, and calling a method on it will
/// always use this instance.
pub struct Buffer<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: Value,
  pub(crate) neovim: Neovim<W>,
}

impl_exttype_traits!(Buffer);

impl<W> IntoVal<Value> for &Buffer<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}
