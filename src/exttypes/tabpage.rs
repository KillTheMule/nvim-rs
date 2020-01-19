//use futures::io::AsyncWrite;
use rmpv::Value;

use crate::{error::CallError, exttypes::Window, rpc::model::IntoVal, Neovim};
use crate::neovim::AsyncWriteSend;

/// A struct representing a neovim tabpage. It is specific to a
/// [`Neovim`](crate::neovim::Neovim) instance, and calling a method on it will
/// always use this instance.
#[derive(Clone)]
pub struct Tabpage<W>
{
  pub(crate) code_data: Value,
  pub(crate) neovim: Neovim<W>,
}

impl<W> Tabpage<W>
where
  W: AsyncWriteSend,
{
  /// since: 1
  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, Box<CallError>> {
    Ok(
      self
        .neovim
        .call("nvim_tabpage_list_wins", call_args![self.code_data.clone()])
        .await?
        .map(|val| {
          if let Value::Array(arr) = val {
            arr
              .into_iter()
              .map(|v| Window::new(v, self.neovim.clone()))
              .collect()
          } else {
            // TODO: Introduce UnexpectedValueError
            panic!("Non-array return value in nvim_tabpage_list_wins!");
          }
        })?,
    )
  }
  /// since: 1
  pub async fn get_win(&self) -> Result<Window<W>, Box<CallError>> {
    Ok(
      self
        .neovim
        .call("nvim_tabpage_get_win", call_args![self.code_data.clone()])
        .await?
        .map(|val| Window::new(val, self.neovim.clone()))?,
    )
  }
}

impl<W> IntoVal<Value> for &Tabpage<W>
{
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}
