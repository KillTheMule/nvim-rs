use super::{Buffer, Tabpage};
use crate::{
  callerror::{map_generic_error, CallError2},
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
  pub async fn get_buf(&self) -> Result<Buffer<W>, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_buf", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(Buffer::new(val, self.requester.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_tabpage(&self) -> Result<Tabpage<W>, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_tabpage", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(Tabpage::new(val, self.requester.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
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
