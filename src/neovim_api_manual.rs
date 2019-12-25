use crate::{
  callerror::CallError,
  rpc::{model::IntoVal},
  neovim::Requester,
  runtime::AsyncWrite,
  Buffer, Tabpage, Window,
};
use rmpv::Value;

impl<W> Requester<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub async fn list_bufs(&self) -> Result<Vec<Buffer<W>>, Box<CallError>> {
    Ok(
      self
        .call("nvim_list_bufs", call_args![])
        .await?
        .map(|val| {
          if let Value::Array(arr) = val {
            arr
              .into_iter()
              .map(|v| Buffer::new(v, self.clone()))
              .collect()
          } else {
            panic!("Non-array response in nvim_list_bufs!");
          }
        })?,
    )
  }

  pub async fn get_current_buf(&self) -> Result<Buffer<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_get_current_buf", call_args![])
        .await?
        .map(|val| Buffer::new(val, self.clone()))?,
    )
  }

  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, Box<CallError>> {
    Ok(
      self
        .call("nvim_list_wins", call_args![])
        .await?
        .map(|val| {
          if let Value::Array(arr) = val {
            arr
              .into_iter()
              .map(|v| Window::new(v, self.clone()))
              .collect()
          } else {
            panic!("Non-array response in nvim_list_bufs!");
          }
        })?,
    )
  }

  pub async fn get_current_win(&self) -> Result<Window<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_get_current_win", call_args![])
        .await?
        .map(|val| Window::new(val, self.clone()))?,
    )
  }

  pub async fn create_buf(
    &self,
    listed: bool,
    scratch: bool,
  ) -> Result<Buffer<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_create_buf", call_args![listed, scratch])
        .await?
        .map(|val| Buffer::new(val, self.clone()))?,
    )
  }

  pub async fn open_win(
    &self,
    buffer: &Buffer<W>,
    enter: bool,
    config: Vec<(Value, Value)>,
  ) -> Result<Window<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_open_win", call_args![buffer, enter, config])
        .await?
        .map(|val| Window::new(val, self.clone()))?,
    )
  }

  pub async fn list_tabpages(&self) -> Result<Vec<Tabpage<W>>, Box<CallError>> {
    Ok(
      self
        .call("nvim_list_tabpages", call_args![])
        .await?
        .map(|val| {
          if let Value::Array(arr) = val {
            arr
              .into_iter()
              .map(|v| Tabpage::new(v, self.clone()))
              .collect()
          } else {
            panic!("Non-array response in nvim_list_bufs!");
          }
        })?,
    )
  }

  pub async fn get_current_tabpage(
    &self,
  ) -> Result<Tabpage<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_get_current_tabpage", call_args![])
        .await?
        .map(|val| Tabpage::new(val, self.clone()))?,
    )
  }
}
