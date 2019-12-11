use crate::Requester;
use rmpv::Value;
use crate::exttypes::Window;
use crate::callerror::{map_generic_error, CallError};
use crate::runtime::AsyncWrite;
use crate::rpc::model::IntoVal;

#[derive(Clone)]
pub struct Tabpage<W> 
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub(crate) code_data: Value,
  pub requester: Requester<W>,
}

impl<W> Tabpage<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  /// since: 1
  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, CallError> {
    self
      .requester
      .call("nvim_tabpage_list_wins", call_args![self.code_data.clone()])
      .await
      .map(|res| {
        if let Value::Array(arr) = res {
          return arr.into_iter().map(|v| Window::new(v, self.requester.clone())).collect();
        } else {
          panic!("Non-array return value in nvim_tabpage_list_wins!");
        }
      })
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_win(&self) -> Result<Window<W>, CallError> {
    self
      .requester
      .call("nvim_tabpage_get_win", call_args![self.code_data.clone()])
      .await
      .map(|res| Window::new(res, self.requester.clone()))
      .map_err(map_generic_error)
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
