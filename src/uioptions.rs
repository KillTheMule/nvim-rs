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
  ExtMultigrid(bool),
  ExtMessages(bool),
  ExtTermcolors(bool),
}

impl UiOption {
  fn to_value(&self) -> (Value, Value) {
    let name_value = self.to_name_value();
    (name_value.0.into(), name_value.1)
  }

  fn to_name_value(&self) -> (&'static str, Value) {
    match *self {
      Self::Rgb(val) => ("rgb", val.into()),
      Self::ExtPopupmenu(val) => ("ext_popupmenu", val.into()),
      Self::ExtTabline(val) => ("ext_tabline", val.into()),
      Self::ExtCmdline(val) => ("ext_cmdline", val.into()),
      Self::ExtWildmenu(val) => ("ext_wildmenu", val.into()),
      Self::ExtLinegrid(val) => ("ext_linegrid", val.into()),
      Self::ExtHlstate(val) => ("ext_hlstate", val.into()),
      Self::ExtMultigrid(val) => ("ext_multigrid", val.into()),
      Self::ExtMessages(val) => ("ext_messages", val.into()),
      Self::ExtTermcolors(val) => ("ext_termcolors", val.into()),
    }
  }
}

#[derive(Default)]
pub struct UiAttachOptions {
  options: Vec<(&'static str, UiOption)>,
}

macro_rules! ui_opt_setters {
  ($( $opt:ident as $set:ident );+ ;) => {
    impl UiAttachOptions {
      $(
        pub fn $set(&mut self, val: bool) -> &mut Self {
          self.set_option(UiOption::$opt(val));
          self
        }
      )+
    }
  }
}

ui_opt_setters! (
  Rgb as set_rgb;
  ExtPopupmenu as set_popupmenu_external;
  ExtTabline as set_tabline_external;
  ExtCmdline as set_cmdline_external;
  ExtWildmenu as set_wildmenu_external;
  ExtLinegrid as set_linegrid_external;
  ExtHlstate as set_hlstate_external;
  ExtMultigrid as set_multigrid_external;
  ExtMessages as set_messages_external;
  ExtTermcolors as set_termcolors_external;
);

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
