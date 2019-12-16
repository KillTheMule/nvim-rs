// Auto generated 2019-12-16 22:17:49.112912
use crate::{runtime::AsyncWrite, Buffer, Tabpage, Window};

use crate::{
  callerror::{map_generic_error, CallError2},
  neovim::*,
  rpc::*,
};

fn map_result<T: FromVal<Value>>(val: Value) -> T {
  T::from_val(val)
}

impl<W> Buffer<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub fn new(code_data: Value, requester: Requester<W>) -> Buffer<W> {
    Buffer {
      code_data,
      requester,
    }
  }

  /// Internal value, that represent type
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }

  /// since: 1
  pub async fn line_count(&self) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_line_count", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 4
  pub async fn attach(
    &self,
    send_buffer: bool,
    opts: Vec<(Value, Value)>,
  ) -> Result<bool, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_attach",
        call_args![self.code_data.clone(), send_buffer, opts],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 4
  pub async fn detach(&self) -> Result<bool, Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_detach", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_lines(
    &self,
    start: i64,
    end: i64,
    strict_indexing: bool,
  ) -> Result<Vec<String>, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_get_lines",
        call_args![self.code_data.clone(), start, end, strict_indexing],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_lines(
    &self,
    start: i64,
    end: i64,
    strict_indexing: bool,
    replacement: Vec<String>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
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
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 5
  pub async fn get_offset(&self, index: i64) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_get_offset",
        call_args![self.code_data.clone(), index],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_var(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_get_var", call_args![self.code_data.clone(), name])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 2
  pub async fn get_changedtick(&self) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_get_changedtick",
        call_args![self.code_data.clone()],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 3
  pub async fn get_keymap(
    &self,
    mode: &str,
  ) -> Result<Vec<Vec<(Value, Value)>>, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_get_keymap",
        call_args![self.code_data.clone(), mode],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 6
  pub async fn set_keymap(
    &self,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_set_keymap",
        call_args![self.code_data.clone(), mode, lhs, rhs, opts],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 6
  pub async fn del_keymap(
    &self,
    mode: &str,
    lhs: &str,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_del_keymap",
        call_args![self.code_data.clone(), mode, lhs],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 4
  pub async fn get_commands(
    &self,
    opts: Vec<(Value, Value)>,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_get_commands",
        call_args![self.code_data.clone(), opts],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_set_var",
        call_args![self.code_data.clone(), name, value],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn del_var(&self, name: &str) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_del_var", call_args![self.code_data.clone(), name])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_option(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_get_option",
        call_args![self.code_data.clone(), name],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_set_option",
        call_args![self.code_data.clone(), name, value],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_number(&self) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_get_number", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_name(&self) -> Result<String, Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_get_name", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_name(&self, name: &str) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_set_name",
        call_args![self.code_data.clone(), name],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 5
  pub async fn is_loaded(&self) -> Result<bool, Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_is_loaded", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn is_valid(&self) -> Result<bool, Box<CallError2>> {
    match self
      .requester
      .call("nvim_buf_is_valid", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_mark(
    &self,
    name: &str,
  ) -> Result<(i64, i64), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_get_mark",
        call_args![self.code_data.clone(), name],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn add_highlight(
    &self,
    ns_id: i64,
    hl_group: &str,
    line: i64,
    col_start: i64,
    col_end: i64,
  ) -> Result<i64, Box<CallError2>> {
    match self
      .requester
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
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 5
  pub async fn clear_namespace(
    &self,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_clear_namespace",
        call_args![self.code_data.clone(), ns_id, line_start, line_end],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn clear_highlight(
    &self,
    ns_id: i64,
    line_start: i64,
    line_end: i64,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_clear_highlight",
        call_args![self.code_data.clone(), ns_id, line_start, line_end],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 5
  pub async fn set_virtual_text(
    &self,
    ns_id: i64,
    line: i64,
    chunks: Vec<Value>,
    opts: Vec<(Value, Value)>,
  ) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_buf_set_virtual_text",
        call_args![self.code_data.clone(), ns_id, line, chunks, opts],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
}

impl<W> Window<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub fn new(code_data: Value, requester: Requester<W>) -> Window<W> {
    Window {
      code_data,
      requester,
    }
  }

  /// Internal value, that represent type
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }

  /// since: 5
  pub async fn set_buf(
    &self,
    buffer: &Buffer<W>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_set_buf",
        call_args![self.code_data.clone(), buffer],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_cursor(&self) -> Result<(i64, i64), Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_cursor", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_cursor(
    &self,
    pos: (i64, i64),
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_set_cursor",
        call_args![self.code_data.clone(), pos],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_height(&self) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_height", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_height(&self, height: i64) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_set_height",
        call_args![self.code_data.clone(), height],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_width(&self) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_width", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_width(&self, width: i64) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_set_width",
        call_args![self.code_data.clone(), width],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_var(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_var", call_args![self.code_data.clone(), name])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_set_var",
        call_args![self.code_data.clone(), name, value],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn del_var(&self, name: &str) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_del_var", call_args![self.code_data.clone(), name])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_option(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_get_option",
        call_args![self.code_data.clone(), name],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_set_option",
        call_args![self.code_data.clone(), name, value],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_position(&self) -> Result<(i64, i64), Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_position", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_number(&self) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_number", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn is_valid(&self) -> Result<bool, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_is_valid", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 6
  pub async fn set_config(
    &self,
    config: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_win_set_config",
        call_args![self.code_data.clone(), config],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 6
  pub async fn get_config(
    &self,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_get_config", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 6
  pub async fn close(&self, force: bool) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call("nvim_win_close", call_args![self.code_data.clone(), force])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
}

impl<W> Tabpage<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub fn new(code_data: Value, requester: Requester<W>) -> Tabpage<W> {
    Tabpage {
      code_data,
      requester,
    }
  }

  /// Internal value, that represent type
  pub fn get_value(&self) -> &Value {
    &self.code_data
  }

  /// since: 1
  pub async fn get_var(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_tabpage_get_var",
        call_args![self.code_data.clone(), name],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_tabpage_set_var",
        call_args![self.code_data.clone(), name, value],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn del_var(&self, name: &str) -> Result<(), Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_tabpage_del_var",
        call_args![self.code_data.clone(), name],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn get_number(&self) -> Result<i64, Box<CallError2>> {
    match self
      .requester
      .call(
        "nvim_tabpage_get_number",
        call_args![self.code_data.clone()],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
  /// since: 1
  pub async fn is_valid(&self) -> Result<bool, Box<CallError2>> {
    match self
      .requester
      .call("nvim_tabpage_is_valid", call_args![self.code_data.clone()])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
}

impl<W> Requester<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub async fn ui_detach(&self) -> Result<(), Box<CallError2>> {
    match self.call("nvim_ui_detach", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn ui_try_resize(
    &self,
    width: i64,
    height: i64,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_ui_try_resize", call_args![width, height])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn ui_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_ui_set_option", call_args![name, value])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn ui_try_resize_grid(
    &self,
    grid: i64,
    width: i64,
    height: i64,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_ui_try_resize_grid", call_args![grid, width, height])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn command(&self, command: &str) -> Result<(), Box<CallError2>> {
    match self.call("nvim_command", call_args![command]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_hl_by_name(
    &self,
    name: &str,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self
      .call("nvim_get_hl_by_name", call_args![name, rgb])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_hl_by_id(
    &self,
    hl_id: i64,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self
      .call("nvim_get_hl_by_id", call_args![hl_id, rgb])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn feedkeys(
    &self,
    keys: &str,
    mode: &str,
    escape_csi: bool,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_feedkeys", call_args![keys, mode, escape_csi])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn input(&self, keys: &str) -> Result<i64, Box<CallError2>> {
    match self.call("nvim_input", call_args![keys]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn input_mouse(
    &self,
    button: &str,
    action: &str,
    modifier: &str,
    grid: i64,
    row: i64,
    col: i64,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call(
        "nvim_input_mouse",
        call_args![button, action, modifier, grid, row, col],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn replace_termcodes(
    &self,
    str: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
  ) -> Result<String, Box<CallError2>> {
    match self
      .call(
        "nvim_replace_termcodes",
        call_args![str, from_part, do_lt, special],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn command_output(
    &self,
    command: &str,
  ) -> Result<String, Box<CallError2>> {
    match self
      .call("nvim_command_output", call_args![command])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn eval(&self, expr: &str) -> Result<Value, Box<CallError2>> {
    match self.call("nvim_eval", call_args![expr]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn execute_lua(
    &self,
    code: &str,
    args: Vec<Value>,
  ) -> Result<Value, Box<CallError2>> {
    match self
      .call("nvim_execute_lua", call_args![code, args])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn call_function(
    &self,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, Box<CallError2>> {
    match self
      .call("nvim_call_function", call_args![fname, args])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn call_dict_function(
    &self,
    dict: Value,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, Box<CallError2>> {
    match self
      .call("nvim_call_dict_function", call_args![dict, fname, args])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn strwidth(&self, text: &str) -> Result<i64, Box<CallError2>> {
    match self.call("nvim_strwidth", call_args![text]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn list_runtime_paths(
    &self,
  ) -> Result<Vec<String>, Box<CallError2>> {
    match self.call("nvim_list_runtime_paths", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_current_dir(
    &self,
    dir: &str,
  ) -> Result<(), Box<CallError2>> {
    match self.call("nvim_set_current_dir", call_args![dir]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_current_line(&self) -> Result<String, Box<CallError2>> {
    match self.call("nvim_get_current_line", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_current_line(
    &self,
    line: &str,
  ) -> Result<(), Box<CallError2>> {
    match self.call("nvim_set_current_line", call_args![line]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn del_current_line(&self) -> Result<(), Box<CallError2>> {
    match self.call("nvim_del_current_line", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_var(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self.call("nvim_get_var", call_args![name]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self.call("nvim_set_var", call_args![name, value]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn del_var(&self, name: &str) -> Result<(), Box<CallError2>> {
    match self.call("nvim_del_var", call_args![name]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_vvar(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self.call("nvim_get_vvar", call_args![name]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_vvar(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self.call("nvim_set_vvar", call_args![name, value]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_option(&self, name: &str) -> Result<Value, Box<CallError2>> {
    match self.call("nvim_get_option", call_args![name]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_set_option", call_args![name, value])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn out_write(&self, str: &str) -> Result<(), Box<CallError2>> {
    match self.call("nvim_out_write", call_args![str]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn err_write(&self, str: &str) -> Result<(), Box<CallError2>> {
    match self.call("nvim_err_write", call_args![str]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn err_writeln(&self, str: &str) -> Result<(), Box<CallError2>> {
    match self.call("nvim_err_writeln", call_args![str]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_current_buf(
    &self,
    buffer: &Buffer<W>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_set_current_buf", call_args![buffer])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_current_win(
    &self,
    window: &Window<W>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_set_current_win", call_args![window])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_current_tabpage(
    &self,
    tabpage: &Tabpage<W>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_set_current_tabpage", call_args![tabpage])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn create_namespace(
    &self,
    name: &str,
  ) -> Result<i64, Box<CallError2>> {
    match self.call("nvim_create_namespace", call_args![name]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_namespaces(
    &self,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self.call("nvim_get_namespaces", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn subscribe(&self, event: &str) -> Result<(), Box<CallError2>> {
    match self.call("nvim_subscribe", call_args![event]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn unsubscribe(&self, event: &str) -> Result<(), Box<CallError2>> {
    match self.call("nvim_unsubscribe", call_args![event]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_color_by_name(
    &self,
    name: &str,
  ) -> Result<i64, Box<CallError2>> {
    match self
      .call("nvim_get_color_by_name", call_args![name])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_color_map(
    &self,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self.call("nvim_get_color_map", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_mode(&self) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self.call("nvim_get_mode", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_keymap(
    &self,
    mode: &str,
  ) -> Result<Vec<Vec<(Value, Value)>>, Box<CallError2>> {
    match self.call("nvim_get_keymap", call_args![mode]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_keymap(
    &self,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call("nvim_set_keymap", call_args![mode, lhs, rhs, opts])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn del_keymap(
    &self,
    mode: &str,
    lhs: &str,
  ) -> Result<(), Box<CallError2>> {
    match self.call("nvim_del_keymap", call_args![mode, lhs]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_commands(
    &self,
    opts: Vec<(Value, Value)>,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self.call("nvim_get_commands", call_args![opts]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_api_info(&self) -> Result<Vec<Value>, Box<CallError2>> {
    match self.call("nvim_get_api_info", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn set_client_info(
    &self,
    name: &str,
    version: Vec<(Value, Value)>,
    typ: &str,
    methods: Vec<(Value, Value)>,
    attributes: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call(
        "nvim_set_client_info",
        call_args![name, version, typ, methods, attributes],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_chan_info(
    &self,
    chan: i64,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self.call("nvim_get_chan_info", call_args![chan]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn list_chans(&self) -> Result<Vec<Value>, Box<CallError2>> {
    match self.call("nvim_list_chans", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn call_atomic(
    &self,
    calls: Vec<Value>,
  ) -> Result<Vec<Value>, Box<CallError2>> {
    match self.call("nvim_call_atomic", call_args![calls]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn parse_expression(
    &self,
    expr: &str,
    flags: &str,
    highlight: bool,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    match self
      .call("nvim_parse_expression", call_args![expr, flags, highlight])
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn list_uis(&self) -> Result<Vec<Value>, Box<CallError2>> {
    match self.call("nvim_list_uis", call_args![]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_proc_children(
    &self,
    pid: i64,
  ) -> Result<Vec<Value>, Box<CallError2>> {
    match self.call("nvim_get_proc_children", call_args![pid]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn get_proc(&self, pid: i64) -> Result<Value, Box<CallError2>> {
    match self.call("nvim_get_proc", call_args![pid]).await? {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }

  pub async fn select_popupmenu_item(
    &self,
    item: i64,
    insert: bool,
    finish: bool,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    match self
      .call(
        "nvim_select_popupmenu_item",
        call_args![item, insert, finish, opts],
      )
      .await?
    {
      Ok(val) => Ok(map_result(val)),
      Err(val) => Err(map_generic_error(val))?,
    }
  }
}

impl<W> Neovim<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub async fn ui_detach(&self) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().ui_detach().await
  }

  pub async fn ui_try_resize(
    &self,
    width: i64,
    height: i64,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().ui_try_resize(width, height).await
  }

  pub async fn ui_set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().ui_set_option(name, value).await
  }

  pub async fn ui_try_resize_grid(
    &self,
    grid: i64,
    width: i64,
    height: i64,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self
      .requester()
      .ui_try_resize_grid(grid, width, height)
      .await
  }

  pub async fn command(&self, command: &str) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().command(command).await
  }

  pub async fn get_hl_by_name(
    &self,
    name: &str,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_hl_by_name(name, rgb).await
  }

  pub async fn get_hl_by_id(
    &self,
    hl_id: i64,
    rgb: bool,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_hl_by_id(hl_id, rgb).await
  }

  pub async fn feedkeys(
    &self,
    keys: &str,
    mode: &str,
    escape_csi: bool,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().feedkeys(keys, mode, escape_csi).await
  }

  pub async fn input(&self, keys: &str) -> Result<i64, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().input(keys).await
  }

  pub async fn input_mouse(
    &self,
    button: &str,
    action: &str,
    modifier: &str,
    grid: i64,
    row: i64,
    col: i64,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self
      .requester()
      .input_mouse(button, action, modifier, grid, row, col)
      .await
  }

  pub async fn replace_termcodes(
    &self,
    str: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
  ) -> Result<String, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self
      .requester()
      .replace_termcodes(str, from_part, do_lt, special)
      .await
  }

  pub async fn command_output(
    &self,
    command: &str,
  ) -> Result<String, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().command_output(command).await
  }

  pub async fn eval(&self, expr: &str) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().eval(expr).await
  }

  pub async fn execute_lua(
    &self,
    code: &str,
    args: Vec<Value>,
  ) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().execute_lua(code, args).await
  }

  pub async fn call_function(
    &self,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().call_function(fname, args).await
  }

  pub async fn call_dict_function(
    &self,
    dict: Value,
    fname: &str,
    args: Vec<Value>,
  ) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().call_dict_function(dict, fname, args).await
  }

  pub async fn strwidth(&self, text: &str) -> Result<i64, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().strwidth(text).await
  }

  pub async fn list_runtime_paths(
    &self,
  ) -> Result<Vec<String>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().list_runtime_paths().await
  }

  pub async fn set_current_dir(
    &self,
    dir: &str,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_current_dir(dir).await
  }

  pub async fn get_current_line(&self) -> Result<String, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_current_line().await
  }

  pub async fn set_current_line(
    &self,
    line: &str,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_current_line(line).await
  }

  pub async fn del_current_line(&self) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().del_current_line().await
  }

  pub async fn get_var(&self, name: &str) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_var(name).await
  }

  pub async fn set_var(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_var(name, value).await
  }

  pub async fn del_var(&self, name: &str) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().del_var(name).await
  }

  pub async fn get_vvar(&self, name: &str) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_vvar(name).await
  }

  pub async fn set_vvar(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_vvar(name, value).await
  }

  pub async fn get_option(&self, name: &str) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_option(name).await
  }

  pub async fn set_option(
    &self,
    name: &str,
    value: Value,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_option(name, value).await
  }

  pub async fn out_write(&self, str: &str) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().out_write(str).await
  }

  pub async fn err_write(&self, str: &str) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().err_write(str).await
  }

  pub async fn err_writeln(&self, str: &str) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().err_writeln(str).await
  }

  pub async fn set_current_buf(
    &self,
    buffer: &Buffer<W>,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_current_buf(buffer).await
  }

  pub async fn set_current_win(
    &self,
    window: &Window<W>,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_current_win(window).await
  }

  pub async fn set_current_tabpage(
    &self,
    tabpage: &Tabpage<W>,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_current_tabpage(tabpage).await
  }

  pub async fn create_namespace(
    &self,
    name: &str,
  ) -> Result<i64, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().create_namespace(name).await
  }

  pub async fn get_namespaces(
    &self,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_namespaces().await
  }

  pub async fn subscribe(&self, event: &str) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().subscribe(event).await
  }

  pub async fn unsubscribe(&self, event: &str) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().unsubscribe(event).await
  }

  pub async fn get_color_by_name(
    &self,
    name: &str,
  ) -> Result<i64, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_color_by_name(name).await
  }

  pub async fn get_color_map(
    &self,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_color_map().await
  }

  pub async fn get_mode(&self) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_mode().await
  }

  pub async fn get_keymap(
    &self,
    mode: &str,
  ) -> Result<Vec<Vec<(Value, Value)>>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_keymap(mode).await
  }

  pub async fn set_keymap(
    &self,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().set_keymap(mode, lhs, rhs, opts).await
  }

  pub async fn del_keymap(
    &self,
    mode: &str,
    lhs: &str,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().del_keymap(mode, lhs).await
  }

  pub async fn get_commands(
    &self,
    opts: Vec<(Value, Value)>,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_commands(opts).await
  }

  pub async fn get_api_info(&self) -> Result<Vec<Value>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_api_info().await
  }

  pub async fn set_client_info(
    &self,
    name: &str,
    version: Vec<(Value, Value)>,
    typ: &str,
    methods: Vec<(Value, Value)>,
    attributes: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self
      .requester()
      .set_client_info(name, version, typ, methods, attributes)
      .await
  }

  pub async fn get_chan_info(
    &self,
    chan: i64,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_chan_info(chan).await
  }

  pub async fn list_chans(&self) -> Result<Vec<Value>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().list_chans().await
  }

  pub async fn call_atomic(
    &self,
    calls: Vec<Value>,
  ) -> Result<Vec<Value>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().call_atomic(calls).await
  }

  pub async fn parse_expression(
    &self,
    expr: &str,
    flags: &str,
    highlight: bool,
  ) -> Result<Vec<(Value, Value)>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self
      .requester()
      .parse_expression(expr, flags, highlight)
      .await
  }

  pub async fn list_uis(&self) -> Result<Vec<Value>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().list_uis().await
  }

  pub async fn get_proc_children(
    &self,
    pid: i64,
  ) -> Result<Vec<Value>, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_proc_children(pid).await
  }

  pub async fn get_proc(&self, pid: i64) -> Result<Value, Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self.requester().get_proc(pid).await
  }

  pub async fn select_popupmenu_item(
    &self,
    item: i64,
    insert: bool,
    finish: bool,
    opts: Vec<(Value, Value)>,
  ) -> Result<(), Box<CallError2>> {
    // TODO: This will clone always, make it a ref
    self
      .requester()
      .select_popupmenu_item(item, insert, finish, opts)
      .await
  }
}
