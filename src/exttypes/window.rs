use super::{Buffer, Tabpage};
use crate::{
  callerror::{map_generic_error, CallError},
  rpc::model::IntoVal,
  runtime::AsyncWrite,
  Requester,
};
use rmpv::Value;

#[derive(Clone)]
pub struct Window<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: Value,
  pub requester: Requester<W>,
}

impl<W> Window<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  /// since: 1
  pub async fn get_buf(&self) -> Result<Buffer<W>, CallError> {
    self
      .requester
      .call("nvim_win_get_buf", call_args![self.code_data.clone()])
      .await
      .map(move |res| Buffer::new(res, self.requester.clone()))
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_tabpage(&self) -> Result<Tabpage<W>, CallError> {
    let r = self.requester.clone();
    self
      .requester
      .call("nvim_win_get_tabpage", call_args![self.code_data.clone()])
      .await
      .map(move |res| Tabpage::new(res, r))
      .map_err(map_generic_error)
  }
}

impl<W> IntoVal<Value> for &Window<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}
