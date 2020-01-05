//! Options for UI implementations
//!
//! This should be used with the manually implemented
//! [`ui_attach`](crate::neovim::Neovim::ui_attach)
use rmpv::Value;

pub enum UiOption {
  Rgb(bool),
  ExtPopupmenu(bool),
  ExtTabline(bool),
  ExtCmdline(bool),
  ExtWildmenu(bool),
  ExtLinegrid(bool),
  ExtHlstate(bool),
}

impl UiOption {
  fn to_value(&self) -> (Value, Value) {
    let name_value = self.to_name_value();
    (name_value.0.into(), name_value.1)
  }

  fn to_name_value(&self) -> (&'static str, Value) {
    match *self {
      UiOption::Rgb(val) => ("rgb", val.into()),
      UiOption::ExtPopupmenu(val) => ("ext_popupmenu", val.into()),
      UiOption::ExtTabline(val) => ("ext_tabline", val.into()),
      UiOption::ExtCmdline(val) => ("ext_cmdline", val.into()),
      UiOption::ExtWildmenu(val) => ("ext_wildmenu", val.into()),
      UiOption::ExtLinegrid(val) => ("ext_linegrid", val.into()),
      UiOption::ExtHlstate(val) => ("ext_hlstate", val.into()),
    }
  }
}

#[derive(Default)]
pub struct UiAttachOptions {
  options: Vec<(&'static str, UiOption)>,
}

impl UiAttachOptions {
  #[must_use]
  pub fn new() -> UiAttachOptions {
    UiAttachOptions {
      options: Vec::new(),
    }
  }

  fn set_option(&mut self, option: UiOption) {
    let name = option.to_name_value();
    let position = self.options.iter().position(|o| o.0 == name.0);

    if let Some(position) = position {
      self.options[position].1 = option;
    } else {
      self.options.push((name.0, option));
    }
  }

  pub fn set_rgb(&mut self, rgb: bool) -> &mut Self {
    self.set_option(UiOption::Rgb(rgb));
    self
  }

  pub fn set_popupmenu_external(
    &mut self,
    popupmenu_external: bool,
  ) -> &mut Self {
    self.set_option(UiOption::ExtPopupmenu(popupmenu_external));
    self
  }

  pub fn set_tabline_external(&mut self, tabline_external: bool) -> &mut Self {
    self.set_option(UiOption::ExtTabline(tabline_external));
    self
  }

  pub fn set_cmdline_external(&mut self, cmdline_external: bool) -> &mut Self {
    self.set_option(UiOption::ExtCmdline(cmdline_external));
    self
  }

  pub fn set_wildmenu_external(
    &mut self,
    wildmenu_external: bool,
  ) -> &mut Self {
    self.set_option(UiOption::ExtWildmenu(wildmenu_external));
    self
  }

  pub fn set_linegrid_external(
    &mut self,
    linegrid_external: bool,
  ) -> &mut Self {
    self.set_option(UiOption::ExtLinegrid(linegrid_external));
    self
  }

  pub fn set_hlstate_external(&mut self, hlstate_external: bool) -> &mut Self {
    self.set_option(UiOption::ExtHlstate(hlstate_external));
    self
  }

  #[must_use]
  pub fn to_value_map(&self) -> Value {
    let map = self.options.iter().map(|o| o.1.to_value()).collect();
    Value::Map(map)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ui_options() {
    let value_map = UiAttachOptions::new()
      .set_rgb(true)
      .set_rgb(false)
      .set_popupmenu_external(true)
      .to_value_map();

    assert_eq!(
      Value::Map(vec![
        ("rgb".into(), false.into()),
        ("ext_popupmenu".into(), true.into()),
      ]),
      value_map
    );
  }
}
