use crate::{
  callerror::{map_generic_error, CallError2},
  exttypes::Window,
  rpc::model::IntoVal,
  runtime::AsyncWrite,
  Requester,
};
use rmpv::Value;

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
  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, Box<CallError2>> {
    match self
      .requester
      .call("nvim_tabpage_list_wins", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => {
        if let Value::Array(arr) = val {
          Ok(
            arr
              .into_iter()
              .map(|v| Window::new(v, self.requester.clone()))
              .collect(),
          )
        } else {
          panic!("Non-array return value in nvim_tabpage_list_wins!");
        }
      }
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_win(&self) -> Result<Window<W>, Box<CallError2>> {
    match self
      .requester
      .call("nvim_tabpage_get_win", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(Window::new(val, self.requester.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
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
