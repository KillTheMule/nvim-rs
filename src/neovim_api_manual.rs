use crate::{
  callerror::{map_generic_error, CallError},
  rpc::{model::IntoVal, Requester},
  runtime::AsyncWrite,
  Buffer, Tabpage, Window,
};
use rmpv::Value;

impl<W> Requester<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub async fn list_bufs(&self) -> Result<Vec<Buffer<W>>, CallError> {
    self
      .call("nvim_list_bufs", call_args![])
      .await
      .map(|res| {
        if let Value::Array(arr) = res {
          return arr
            .into_iter()
            .map(|v| Buffer::new(v, self.clone()))
            .collect();
        } else {
          panic!("Non-array response in nvim_list_bufs!");
        }
      })
      .map_err(map_generic_error)
  }

  pub async fn get_current_buf(&self) -> Result<Buffer<W>, CallError> {
    self
      .call("nvim_get_current_buf", call_args![])
      .await
      .map(|res| Buffer::new(res, self.clone()))
      .map_err(map_generic_error)
  }

  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, CallError> {
    self
      .call("nvim_list_wins", call_args![])
      .await
      .map(|res| {
        if let Value::Array(arr) = res {
          return arr
            .into_iter()
            .map(|v| Window::new(v, self.clone()))
            .collect();
        } else {
          panic!("Non-array return value in nvim_list_wins!");
        }
      })
      .map_err(map_generic_error)
  }

  pub async fn get_current_win(&self) -> Result<Window<W>, CallError> {
    self
      .call("nvim_get_current_win", call_args![])
      .await
      .map(|res| Window::new(res, self.clone()))
      .map_err(map_generic_error)
  }

  pub async fn create_buf(
    &self,
    listed: bool,
    scratch: bool,
  ) -> Result<Buffer<W>, CallError> {
    self
      .call("nvim_create_buf", call_args![listed, scratch])
      .await
      .map(|res| Buffer::new(res, self.clone()))
      .map_err(map_generic_error)
  }

  pub async fn open_win(
    &self,
    buffer: &Buffer<W>,
    enter: bool,
    config: Vec<(Value, Value)>,
  ) -> Result<Window<W>, CallError> {
    self
      .call("nvim_open_win", call_args![buffer, enter, config])
      .await
      .map(|res| Window::new(res, self.clone()))
      .map_err(map_generic_error)
  }

  pub async fn list_tabpages(&self) -> Result<Vec<Tabpage<W>>, CallError> {
    self
      .call("nvim_list_tabpages", call_args![])
      .await
      .map(|res| {
        if let Value::Array(arr) = res {
          return arr
            .into_iter()
            .map(|v| Tabpage::new(v, self.clone()))
            .collect();
        } else {
          panic!("Non-array return value in nvim_list_tabpages!");
        }
      })
      .map_err(map_generic_error)
  }

  pub async fn get_current_tabpage(&self) -> Result<Tabpage<W>, CallError> {
    self
      .call("nvim_get_current_tabpage", call_args![])
      .await
      .map(|res| Tabpage::new(res, self.clone()))
      .map_err(map_generic_error)
  }
}
