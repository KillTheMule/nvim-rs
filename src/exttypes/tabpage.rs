use crate::{
  callerror::CallError, exttypes::Window, rpc::model::IntoVal,
  runtime::AsyncWrite, Neovim,
};
use rmpv::Value;

#[derive(Clone)]
pub struct Tabpage<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub(crate) code_data: Value,
  pub requester: Neovim<W>,
}

impl<W> Tabpage<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  /// since: 1
  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, Box<CallError>> {
    Ok(
      self
        .requester
        .call("nvim_tabpage_list_wins", call_args![self.code_data.clone()])
        .await?
        .map(|val| {
          if let Value::Array(arr) = val {
            arr
              .into_iter()
              .map(|v| Window::new(v, self.requester.clone()))
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
        .requester
        .call("nvim_tabpage_get_win", call_args![self.code_data.clone()])
        .await?
        .map(|val| Window::new(val, self.requester.clone()))?,
    )
  }
}

impl<W> IntoVal<Value> for &Tabpage<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}
