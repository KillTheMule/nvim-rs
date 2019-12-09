use rmpv::Value;
use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CallError {
  GenericError(String),
  NeovimError(i64, String),
}

impl fmt::Display for CallError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      CallError::GenericError(ref s) => write!(f, "Unknown error type: {}", s),
      CallError::NeovimError(id, ref s) => write!(f, "{} - {}", id, s),
    }
  }
}

impl Error for CallError {
  fn description(&self) -> &str {
    match *self {
      CallError::GenericError(ref s) => s,
      CallError::NeovimError(_, ref s) => s,
    }
  }
}

#[doc(hidden)]
pub fn map_generic_error(err: Value) -> CallError {
  match err {
    Value::String(val) => {
      CallError::GenericError(val.as_str().unwrap().to_owned())
    }
    Value::Array(arr) => {
      if arr.len() == 2 {
        match (&arr[0], &arr[1]) {
          (&Value::Integer(ref id), &Value::String(ref val)) => {
            CallError::NeovimError(
              id.as_i64().unwrap(),
              val.as_str().unwrap().to_owned(),
            )
          }
          _ => CallError::GenericError(format!("{:?}", arr)),
        }
      } else {
        CallError::GenericError(format!("{:?}", arr))
      }
    }
    val => CallError::GenericError(format!("{:?}", val)),
  }
}
