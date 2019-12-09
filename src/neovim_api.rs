// Auto generated 2019-11-30 22:14:46.771419
use crate::runtime::AsyncWrite;

use crate::{
  callerror::{map_generic_error, CallError},
  neovim::*,
  rpc::*,
};

fn map_result<T: FromVal<Value>>(val: Value) -> T {
  T::from_val(val)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Buffer {
  code_data: Value,
}

impl Buffer {
  pub fn new(code_data: Value) -> Buffer {
    Buffer { code_data }
  }

  /// Internal value, that represent type
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }

  /// since: 1
  pub async fn line_count<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<i64, CallError>
  where
    W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_line_count", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 4
  pub async fn attach<W>(
    &self,
    neovim: &Neovim<W>,
    send_buffer: bool,
    opts: Vec<(Value, Value)>,
  ) -> Result<bool, CallError>
  where
    W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_attach",
        call_args![self.code_data.clone(), send_buffer, opts],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 4
  pub async fn detach<W>(&self, neovim: &Neovim<W>) -> Result<bool, CallError>
  where
    W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_detach", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_lines<W>(
    &self,
    neovim: &Neovim<W>,
    start: i64,
    end: i64,
    strict_indexing: bool,
  ) -> Result<Vec<String>, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_get_lines",
        call_args![self.code_data.clone(), start, end, strict_indexing],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_lines<W>(
    &self,
    neovim: &Neovim<W>,
    start: i64,
    end: i64,
    strict_indexing: bool,
    replacement: Vec<String>,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_set_lines",
        call_args![
          self.code_data.clone(),
          start,
          end,
          strict_indexing,
          replacement
        ],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 5
  pub async fn get_offset<W>(
    &self,
    neovim: &Neovim<W>,
    index: i64,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_get_offset",
        call_args![self.code_data.clone(), index],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<Value, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_get_var", call_args![self.code_data.clone(), name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 2
  pub async fn get_changedtick<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_get_changedtick",
        call_args![self.code_data.clone()],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 3
  pub async fn get_keymap<W>(
    &self,
    neovim: &Neovim<W>,
    mode: &str,
  ) -> Result<Vec<Vec<(Value, Value)>>, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_get_keymap",
        call_args![self.code_data.clone(), mode],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 6
  pub async fn set_keymap<W>(
    &self,
    neovim: &Neovim<W>,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_set_keymap",
        call_args![self.code_data.clone(), mode, lhs, rhs, opts],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 6
  pub async fn del_keymap<W>(
    &self,
    neovim: &Neovim<W>,
    mode: &str,
    lhs: &str,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_del_keymap",
        call_args![self.code_data.clone(), mode, lhs],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 4
  pub async fn get_commands<W>(
    &self,
    neovim: &Neovim<W>,
    opts: Vec<(Value, Value)>,
  ) -> Result<Vec<(Value, Value)>, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_get_commands",
        call_args![self.code_data.clone(), opts],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
    value: Value,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_set_var",
        call_args![self.code_data.clone(), name, value],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn del_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_del_var", call_args![self.code_data.clone(), name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_option<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<Value, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_get_option",
        call_args![self.code_data.clone(), name],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_option<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
    value: Value,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_set_option",
        call_args![self.code_data.clone(), name, value],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_number<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_get_number", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_name<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<String, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_get_name", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_name<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_set_name",
        call_args![self.code_data.clone(), name],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 5
  pub async fn is_loaded<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<bool, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_is_loaded", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn is_valid<W>(&self, neovim: &Neovim<W>) -> Result<bool, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_buf_is_valid", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_mark<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<(i64, i64), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_get_mark",
        call_args![self.code_data.clone(), name],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn add_highlight<W>(
    &self,
    neovim: &Neovim<W>,
    ns_id: i64,
    hl_group: &str,
    line: i64,
    col_start: i64,
    col_end: i64,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_add_highlight",
        call_args![
          self.code_data.clone(),
          ns_id,
          hl_group,
          line,
          col_start,
          col_end
        ],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 5
  pub async fn clear_namespace<W>(
    &self,
    neovim: &Neovim<W>,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_clear_namespace",
        call_args![self.code_data.clone(), ns_id, line_start, line_end],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn clear_highlight<W>(
    &self,
    neovim: &Neovim<W>,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_clear_highlight",
        call_args![self.code_data.clone(), ns_id, line_start, line_end],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 5
  pub async fn set_virtual_text<W>(
    &self,
    neovim: &Neovim<W>,
    ns_id: i64,
    line: i64,
    chunks: Vec<Value>,
    opts: Vec<(Value, Value)>,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_buf_set_virtual_text",
        call_args![self.code_data.clone(), ns_id, line, chunks, opts],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Window {
  code_data: Value,
}

impl Window {
  pub fn new(code_data: Value) -> Window {
    Window { code_data }
  }

  /// Internal value, that represent type
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }

  /// since: 1
  pub async fn get_buf<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<Buffer, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_buf", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 5
  pub async fn set_buf<W>(
    &self,
    neovim: &Neovim<W>,
    buffer: &Buffer,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_set_buf",
        call_args![self.code_data.clone(), buffer],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_cursor<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<(i64, i64), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_cursor", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_cursor<W>(
    &self,
    neovim: &Neovim<W>,
    pos: (i64, i64),
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_set_cursor",
        call_args![self.code_data.clone(), pos],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_height<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_height", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_height<W>(
    &self,
    neovim: &Neovim<W>,
    height: i64,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_set_height",
        call_args![self.code_data.clone(), height],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_width<W>(&self, neovim: &Neovim<W>) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_width", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_width<W>(
    &self,
    neovim: &Neovim<W>,
    width: i64,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_set_width",
        call_args![self.code_data.clone(), width],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<Value, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_var", call_args![self.code_data.clone(), name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
    value: Value,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_set_var",
        call_args![self.code_data.clone(), name, value],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn del_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_del_var", call_args![self.code_data.clone(), name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_option<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<Value, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_get_option",
        call_args![self.code_data.clone(), name],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_option<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
    value: Value,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_set_option",
        call_args![self.code_data.clone(), name, value],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_position<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<(i64, i64), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_position", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_tabpage<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<Tabpage, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_tabpage", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_number<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_number", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn is_valid<W>(&self, neovim: &Neovim<W>) -> Result<bool, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_is_valid", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 6
  pub async fn set_config<W>(
    &self,
    neovim: &Neovim<W>,
    config: Vec<(Value, Value)>,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_win_set_config",
        call_args![self.code_data.clone(), config],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 6
  pub async fn get_config<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<Vec<(Value, Value)>, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_get_config", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 6
  pub async fn close<W>(
    &self,
    neovim: &Neovim<W>,
    force: bool,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_win_close", call_args![self.code_data.clone(), force])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Tabpage {
  code_data: Value,
}

impl Tabpage {
  pub fn new(code_data: Value) -> Tabpage {
    Tabpage { code_data }
  }

  /// Internal value, that represent type
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }

  /// since: 1
  pub async fn list_wins<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<Vec<Window>, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_tabpage_list_wins", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<Value, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_tabpage_get_var",
        call_args![self.code_data.clone(), name],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn set_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
    value: Value,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_tabpage_set_var",
        call_args![self.code_data.clone(), name, value],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn del_var<W>(
    &self,
    neovim: &Neovim<W>,
    name: &str,
  ) -> Result<(), CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_tabpage_del_var",
        call_args![self.code_data.clone(), name],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_win<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<Window, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_tabpage_get_win", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn get_number<W>(
    &self,
    neovim: &Neovim<W>,
  ) -> Result<i64, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call(
        "nvim_tabpage_get_number",
        call_args![self.code_data.clone()],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
  /// since: 1
  pub async fn is_valid<W>(&self, neovim: &Neovim<W>) -> Result<bool, CallError>
  where
        W: AsyncWrite + Send + Unpin + 'static,
  {
    neovim
      .call("nvim_tabpage_is_valid", call_args![self.code_data.clone()])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
}

impl FromVal<Value> for Buffer {
  fn from_val(val: Value) -> Self {
    Buffer::new(val)
  }
}

impl<'a> IntoVal<Value> for &'a Buffer {
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}
impl FromVal<Value> for Window {
  fn from_val(val: Value) -> Self {
    Window::new(val)
  }
}

impl<'a> IntoVal<Value> for &'a Window {
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}
impl FromVal<Value> for Tabpage {
  fn from_val(val: Value) -> Self {
    Tabpage::new(val)
  }
}

impl<'a> IntoVal<Value> for &'a Tabpage {
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}

impl<W> Requester<W>
where
      W: AsyncWrite + Send + Unpin + 'static,
{
  pub async fn ui_detach(&self) -> Result<(), CallError> {
    self
      .call("nvim_ui_detach", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn ui_try_resize(
    &self,
    width: i64,
    height: i64,
  ) -> Result<(), CallError> {
    self
      .call("nvim_ui_try_resize", call_args![width, height])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn ui_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_ui_set_option", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn ui_try_resize_grid(
    &self,
    grid: i64,
    width: i64,
    height: i64,
  ) -> Result<(), CallError> {
    self
      .call("nvim_ui_try_resize_grid", call_args![grid, width, height])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn command(&self, command: &str) -> Result<(), CallError> {
    self
      .call("nvim_command", call_args![command])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_hl_by_name(
    &self,
    name: &str,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_hl_by_name", call_args![name, rgb])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_hl_by_id(
    &self,
    hl_id: i64,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_hl_by_id", call_args![hl_id, rgb])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn feedkeys(
    &self,
    keys: &str,
    mode: &str,
    escape_csi: bool,
  ) -> Result<(), CallError> {
    self
      .call("nvim_feedkeys", call_args![keys, mode, escape_csi])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn input(&self, keys: &str) -> Result<i64, CallError> {
    self
      .call("nvim_input", call_args![keys])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn input_mouse(
    &self,
    button: &str,
    action: &str,
    modifier: &str,
    grid: i64,
    row: i64,
    col: i64,
  ) -> Result<(), CallError> {
    self
      .call(
        "nvim_input_mouse",
        call_args![button, action, modifier, grid, row, col],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn replace_termcodes(
    &self,
    str: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
  ) -> Result<String, CallError> {
    self
      .call(
        "nvim_replace_termcodes",
        call_args![str, from_part, do_lt, special],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn command_output(
    &self,
    command: &str,
  ) -> Result<String, CallError> {
    self
      .call("nvim_command_output", call_args![command])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn eval(&self, expr: &str) -> Result<Value, CallError> {
    self
      .call("nvim_eval", call_args![expr])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn execute_lua(
    &self,
    code: &str,
    args: Vec<Value>,
  ) -> Result<Value, CallError> {
    self
      .call("nvim_execute_lua", call_args![code, args])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn call_function(
    &self,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, CallError> {
    self
      .call("nvim_call_function", call_args![fname, args])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn call_dict_function(
    &self,
    dict: Value,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, CallError> {
    self
      .call("nvim_call_dict_function", call_args![dict, fname, args])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn strwidth(&self, text: &str) -> Result<i64, CallError> {
    self
      .call("nvim_strwidth", call_args![text])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_runtime_paths(&self) -> Result<Vec<String>, CallError> {
    self
      .call("nvim_list_runtime_paths", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_dir(&self, dir: &str) -> Result<(), CallError> {
    self
      .call("nvim_set_current_dir", call_args![dir])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_line(&self) -> Result<String, CallError> {
    self
      .call("nvim_get_current_line", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_line(&self, line: &str) -> Result<(), CallError> {
    self
      .call("nvim_set_current_line", call_args![line])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn del_current_line(&self) -> Result<(), CallError> {
    self
      .call("nvim_del_current_line", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_var(&self, name: &str) -> Result<Value, CallError> {
    self
      .call("nvim_get_var", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_var", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn del_var(&self, name: &str) -> Result<(), CallError> {
    self
      .call("nvim_del_var", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_vvar(&self, name: &str) -> Result<Value, CallError> {
    self
      .call("nvim_get_vvar", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_vvar(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_vvar", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_option(&self, name: &str) -> Result<Value, CallError> {
    self
      .call("nvim_get_option", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_option", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn out_write(&self, str: &str) -> Result<(), CallError> {
    self
      .call("nvim_out_write", call_args![str])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn err_write(&self, str: &str) -> Result<(), CallError> {
    self
      .call("nvim_err_write", call_args![str])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn err_writeln(&self, str: &str) -> Result<(), CallError> {
    self
      .call("nvim_err_writeln", call_args![str])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_bufs(&self) -> Result<Vec<Buffer>, CallError> {
    self
      .call("nvim_list_bufs", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_buf(&self) -> Result<Buffer, CallError> {
    self
      .call("nvim_get_current_buf", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_buf(
    &self,
    buffer: &Buffer,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_current_buf", call_args![buffer])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_wins(&self) -> Result<Vec<Window>, CallError> {
    self
      .call("nvim_list_wins", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_win(&self) -> Result<Window, CallError> {
    self
      .call("nvim_get_current_win", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_win(
    &self,
    window: &Window,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_current_win", call_args![window])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn create_buf(
    &self,
    listed: bool,
    scratch: bool,
  ) -> Result<Buffer, CallError> {
    self
      .call("nvim_create_buf", call_args![listed, scratch])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn open_win(
    &self,
    buffer: &Buffer,
    enter: bool,
    config: Vec<(Value, Value)>,
  ) -> Result<Window, CallError> {
    self
      .call("nvim_open_win", call_args![buffer, enter, config])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_tabpages(&self) -> Result<Vec<Tabpage>, CallError> {
    self
      .call("nvim_list_tabpages", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_tabpage(&self) -> Result<Tabpage, CallError> {
    self
      .call("nvim_get_current_tabpage", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_tabpage(
    &self,
    tabpage: &Tabpage,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_current_tabpage", call_args![tabpage])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn create_namespace(&self, name: &str) -> Result<i64, CallError> {
    self
      .call("nvim_create_namespace", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_namespaces(&self) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_namespaces", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn subscribe(&self, event: &str) -> Result<(), CallError> {
    self
      .call("nvim_subscribe", call_args![event])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn unsubscribe(&self, event: &str) -> Result<(), CallError> {
    self
      .call("nvim_unsubscribe", call_args![event])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_color_by_name(&self, name: &str) -> Result<i64, CallError> {
    self
      .call("nvim_get_color_by_name", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_color_map(&self) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_color_map", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_mode(&self) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_mode", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_keymap(
    &self,
    mode: &str,
  ) -> Result<Vec<Vec<(Value, Value)>>, CallError> {
    self
      .call("nvim_get_keymap", call_args![mode])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_keymap(
    &self,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_keymap", call_args![mode, lhs, rhs, opts])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn del_keymap(
    &self,
    mode: &str,
    lhs: &str,
  ) -> Result<(), CallError> {
    self
      .call("nvim_del_keymap", call_args![mode, lhs])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_commands(
    &self,
    opts: Vec<(Value, Value)>,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_commands", call_args![opts])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_api_info(&self) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_get_api_info", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_client_info(
    &self,
    name: &str,
    version: Vec<(Value, Value)>,
    typ: &str,
    methods: Vec<(Value, Value)>,
    attributes: Vec<(Value, Value)>,
  ) -> Result<(), CallError> {
    self
      .call(
        "nvim_set_client_info",
        call_args![name, version, typ, methods, attributes],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_chan_info(
    &self,
    chan: i64,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_chan_info", call_args![chan])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_chans(&self) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_list_chans", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn call_atomic(
    &self,
    calls: Vec<Value>,
  ) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_call_atomic", call_args![calls])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn parse_expression(
    &self,
    expr: &str,
    flags: &str,
    highlight: bool,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_parse_expression", call_args![expr, flags, highlight])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_uis(&self) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_list_uis", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_proc_children(
    &self,
    pid: i64,
  ) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_get_proc_children", call_args![pid])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_proc(&self, pid: i64) -> Result<Value, CallError> {
    self
      .call("nvim_get_proc", call_args![pid])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn select_popupmenu_item(
    &self,
    item: i64,
    insert: bool,
    finish: bool,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), CallError> {
    self
      .call(
        "nvim_select_popupmenu_item",
        call_args![item, insert, finish, opts],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
}

impl<W> Neovim<W>
where
      W: AsyncWrite + Send + Unpin + 'static,
{
  pub async fn ui_detach(&self) -> Result<(), CallError> {
    self
      .call("nvim_ui_detach", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn ui_try_resize(
    &self,
    width: i64,
    height: i64,
  ) -> Result<(), CallError> {
    self
      .call("nvim_ui_try_resize", call_args![width, height])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn ui_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_ui_set_option", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn ui_try_resize_grid(
    &self,
    grid: i64,
    width: i64,
    height: i64,
  ) -> Result<(), CallError> {
    self
      .call("nvim_ui_try_resize_grid", call_args![grid, width, height])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn command(&self, command: &str) -> Result<(), CallError> {
    self
      .call("nvim_command", call_args![command])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_hl_by_name(
    &self,
    name: &str,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_hl_by_name", call_args![name, rgb])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_hl_by_id(
    &self,
    hl_id: i64,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_hl_by_id", call_args![hl_id, rgb])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn feedkeys(
    &self,
    keys: &str,
    mode: &str,
    escape_csi: bool,
  ) -> Result<(), CallError> {
    self
      .call("nvim_feedkeys", call_args![keys, mode, escape_csi])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn input(&self, keys: &str) -> Result<i64, CallError> {
    self
      .call("nvim_input", call_args![keys])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn input_mouse(
    &self,
    button: &str,
    action: &str,
    modifier: &str,
    grid: i64,
    row: i64,
    col: i64,
  ) -> Result<(), CallError> {
    self
      .call(
        "nvim_input_mouse",
        call_args![button, action, modifier, grid, row, col],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn replace_termcodes(
    &self,
    str: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
  ) -> Result<String, CallError> {
    self
      .call(
        "nvim_replace_termcodes",
        call_args![str, from_part, do_lt, special],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn command_output(
    &self,
    command: &str,
  ) -> Result<String, CallError> {
    self
      .call("nvim_command_output", call_args![command])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn eval(&self, expr: &str) -> Result<Value, CallError> {
    self
      .call("nvim_eval", call_args![expr])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn execute_lua(
    &self,
    code: &str,
    args: Vec<Value>,
  ) -> Result<Value, CallError> {
    self
      .call("nvim_execute_lua", call_args![code, args])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn call_function(
    &self,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, CallError> {
    self
      .call("nvim_call_function", call_args![fname, args])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn call_dict_function(
    &self,
    dict: Value,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, CallError> {
    self
      .call("nvim_call_dict_function", call_args![dict, fname, args])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn strwidth(&self, text: &str) -> Result<i64, CallError> {
    self
      .call("nvim_strwidth", call_args![text])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_runtime_paths(&self) -> Result<Vec<String>, CallError> {
    self
      .call("nvim_list_runtime_paths", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_dir(&self, dir: &str) -> Result<(), CallError> {
    self
      .call("nvim_set_current_dir", call_args![dir])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_line(&self) -> Result<String, CallError> {
    self
      .call("nvim_get_current_line", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_line(&self, line: &str) -> Result<(), CallError> {
    self
      .call("nvim_set_current_line", call_args![line])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn del_current_line(&self) -> Result<(), CallError> {
    self
      .call("nvim_del_current_line", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_var(&self, name: &str) -> Result<Value, CallError> {
    self
      .call("nvim_get_var", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_var", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn del_var(&self, name: &str) -> Result<(), CallError> {
    self
      .call("nvim_del_var", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_vvar(&self, name: &str) -> Result<Value, CallError> {
    self
      .call("nvim_get_vvar", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_vvar(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_vvar", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_option(&self, name: &str) -> Result<Value, CallError> {
    self
      .call("nvim_get_option", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_option", call_args![name, value])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn out_write(&self, str: &str) -> Result<(), CallError> {
    self
      .call("nvim_out_write", call_args![str])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn err_write(&self, str: &str) -> Result<(), CallError> {
    self
      .call("nvim_err_write", call_args![str])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn err_writeln(&self, str: &str) -> Result<(), CallError> {
    self
      .call("nvim_err_writeln", call_args![str])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_bufs(&self) -> Result<Vec<Buffer>, CallError> {
    self
      .call("nvim_list_bufs", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_buf(&self) -> Result<Buffer, CallError> {
    self
      .call("nvim_get_current_buf", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_buf(
    &self,
    buffer: &Buffer,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_current_buf", call_args![buffer])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_wins(&self) -> Result<Vec<Window>, CallError> {
    self
      .call("nvim_list_wins", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_win(&self) -> Result<Window, CallError> {
    self
      .call("nvim_get_current_win", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_win(
    &self,
    window: &Window,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_current_win", call_args![window])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn create_buf(
    &self,
    listed: bool,
    scratch: bool,
  ) -> Result<Buffer, CallError> {
    self
      .call("nvim_create_buf", call_args![listed, scratch])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn open_win(
    &self,
    buffer: &Buffer,
    enter: bool,
    config: Vec<(Value, Value)>,
  ) -> Result<Window, CallError> {
    self
      .call("nvim_open_win", call_args![buffer, enter, config])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_tabpages(&self) -> Result<Vec<Tabpage>, CallError> {
    self
      .call("nvim_list_tabpages", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_current_tabpage(&self) -> Result<Tabpage, CallError> {
    self
      .call("nvim_get_current_tabpage", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_current_tabpage(
    &self,
    tabpage: &Tabpage,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_current_tabpage", call_args![tabpage])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn create_namespace(&self, name: &str) -> Result<i64, CallError> {
    self
      .call("nvim_create_namespace", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_namespaces(&self) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_namespaces", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn subscribe(&self, event: &str) -> Result<(), CallError> {
    self
      .call("nvim_subscribe", call_args![event])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn unsubscribe(&self, event: &str) -> Result<(), CallError> {
    self
      .call("nvim_unsubscribe", call_args![event])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_color_by_name(&self, name: &str) -> Result<i64, CallError> {
    self
      .call("nvim_get_color_by_name", call_args![name])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_color_map(&self) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_color_map", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_mode(&self) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_mode", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_keymap(
    &self,
    mode: &str,
  ) -> Result<Vec<Vec<(Value, Value)>>, CallError> {
    self
      .call("nvim_get_keymap", call_args![mode])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_keymap(
    &self,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), CallError> {
    self
      .call("nvim_set_keymap", call_args![mode, lhs, rhs, opts])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn del_keymap(
    &self,
    mode: &str,
    lhs: &str,
  ) -> Result<(), CallError> {
    self
      .call("nvim_del_keymap", call_args![mode, lhs])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_commands(
    &self,
    opts: Vec<(Value, Value)>,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_commands", call_args![opts])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_api_info(&self) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_get_api_info", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn set_client_info(
    &self,
    name: &str,
    version: Vec<(Value, Value)>,
    typ: &str,
    methods: Vec<(Value, Value)>,
    attributes: Vec<(Value, Value)>,
  ) -> Result<(), CallError> {
    self
      .call(
        "nvim_set_client_info",
        call_args![name, version, typ, methods, attributes],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_chan_info(
    &self,
    chan: i64,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_get_chan_info", call_args![chan])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_chans(&self) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_list_chans", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn call_atomic(
    &self,
    calls: Vec<Value>,
  ) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_call_atomic", call_args![calls])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn parse_expression(
    &self,
    expr: &str,
    flags: &str,
    highlight: bool,
  ) -> Result<Vec<(Value, Value)>, CallError> {
    self
      .call("nvim_parse_expression", call_args![expr, flags, highlight])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn list_uis(&self) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_list_uis", call_args![])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_proc_children(
    &self,
    pid: i64,
  ) -> Result<Vec<Value>, CallError> {
    self
      .call("nvim_get_proc_children", call_args![pid])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn get_proc(&self, pid: i64) -> Result<Value, CallError> {
    self
      .call("nvim_get_proc", call_args![pid])
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }

  pub async fn select_popupmenu_item(
    &self,
    item: i64,
    insert: bool,
    finish: bool,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), CallError> {
    self
      .call(
        "nvim_select_popupmenu_item",
        call_args![item, insert, finish, opts],
      )
      .await
      .map(map_result)
      .map_err(map_generic_error)
  }
}
