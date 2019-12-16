use crate::{
  callerror::{map_generic_error, CallError2},
  rpc::{model::IntoVal, Requester},
  runtime::AsyncWrite,
  Buffer, Tabpage, Window,
};
use rmpv::Value;

impl<W> Requester<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub async fn list_bufs(&self) -> Result<Vec<Buffer<W>>, Box<CallError2>> {
    match self.call("nvim_list_bufs", call_args![]).await? {
      Ok(val) => {
        if let Value::Array(arr) = val {
          Ok(
            arr
              .into_iter()
              .map(|v| Buffer::new(v, self.clone()))
              .collect(),
          )
        } else {
          panic!("Non-array response in nvim_list_bufs!");
        }
      }
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_current_buf(&self) -> Result<Buffer<W>, Box<CallError2>> {
    match self.call("nvim_get_current_buf", call_args![]).await? {
      Ok(val) => Ok(Buffer::new(val, self.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, Box<CallError2>> {
    match self.call("nvim_list_wins", call_args![]).await? {
      Ok(val) => {
        if let Value::Array(arr) = val {
          Ok(
            arr
              .into_iter()
              .map(|v| Window::new(v, self.clone()))
              .collect(),
          )
        } else {
          panic!("Non-array response in nvim_list_bufs!");
        }
      }
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_current_win(&self) -> Result<Window<W>, Box<CallError2>> {
    match self.call("nvim_get_current_win", call_args![]).await? {
      Ok(val) => Ok(Window::new(val, self.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn create_buf(
    &self,
    listed: bool,
    scratch: bool,
  ) -> Result<Buffer<W>, Box<CallError2>> {
    match self
      .call("nvim_create_buf", call_args![listed, scratch])
      .await?
    {
      Ok(val) => Ok(Buffer::new(val, self.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn open_win(
    &self,
    buffer: &Buffer<W>,
    enter: bool,
    config: Vec<(Value, Value)>,
  ) -> Result<Window<W>, Box<CallError2>> {
    match self
      .call("nvim_open_win", call_args![buffer, enter, config])
      .await?
    {
      Ok(val) => Ok(Window::new(val, self.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn list_tabpages(
    &self,
  ) -> Result<Vec<Tabpage<W>>, Box<CallError2>> {
    match self.call("nvim_list_tabpages", call_args![]).await? {
      Ok(val) => {
        if let Value::Array(arr) = val {
          Ok(
            arr
              .into_iter()
              .map(|v| Tabpage::new(v, self.clone()))
              .collect(),
          )
        } else {
          panic!("Non-array response in nvim_list_bufs!");
        }
      }
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_current_tabpage(
    &self,
  ) -> Result<Tabpage<W>, Box<CallError2>> {
    match self.call("nvim_get_current_tabpage", call_args![]).await? {
      Ok(val) => Ok(Tabpage::new(val, self.clone())),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
}
