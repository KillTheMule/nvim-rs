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

use rmpv::decode::Error as RmpvDecodeError;
use std::{fmt::Display, ops::RangeInclusive};

#[derive(Debug, PartialEq, Clone)]
pub enum InvalidMessageError {
  /// The value read was not an array
  NotAnArray(Value),
  /// WrongArrayLength(should, is) means that the array should have length in
  /// the range `should`, but has length `is`
  WrongArrayLength(RangeInclusive<u64>, u64),
  // TODO: Make a method on value that returns the value on error, so we can
  // recover the non-decodable value here.
  /// The first array element (=the message type) was not decodable into a u64
  InvalidMessageType,
  /// The first array element (=the message type) was decodable into a u64
  /// larger than 2
  UnknownMessageType(u64),
  /// The params of a request or notification weren't an array
  InvalidParams(Value, String),
  // TODO: Make a method on value that returns the value on error, so we can
  // recover the non-decodable value here.
  /// The method name of a notification was not a string
  InvalidNotificationName,
  // TODO: Make a method on value that returns the value on error, so we can
  // recover the non-decodable value here.
  /// The method name of a request was not a string
  InvalidRequestName(u64),
  // TODO: Make a method on value that returns the value on error, so we can
  // recover the non-decodable value here.
  /// The msgid of a request or response was not decodable into a u64
  InvalidMsgid,
}

impl Error for InvalidMessageError {}

impl Display for InvalidMessageError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    use InvalidMessageError::*;

    match self {
      NotAnArray(val) => write!(fmt, "Value not an Array: '{}'", val),
      WrongArrayLength(should, is) => write!(
        fmt,
        "Array should have length {:?}, has length {}",
        should, is
      ),
      InvalidMessageType => write!(fmt, "Message type not decodable into u64"),
      UnknownMessageType(m) => {
        write!(fmt, "Message type {} is not 0, 1 or 2", m)
      }
      InvalidParams(val, s) => {
        write!(fmt, "Params of method '{}' not an Array: '{}'", s, val)
      }
      InvalidNotificationName => write!(fmt, "Notification name invalid utf8"),
      InvalidRequestName(id) => {
        write!(fmt, "Request id {}: name invalid utf8", id)
      }
      InvalidMsgid => write!(fmt, "Msgid of message not decodable into u64"),
    }
  }
}

#[derive(Debug)]
pub enum DecodeError {
  InvalidRead(RmpvDecodeError),
  InvalidMessage(InvalidMessageError),
}

impl Error for DecodeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      DecodeError::InvalidRead(ref e) => Some(e),
      DecodeError::InvalidMessage(ref e) => Some(e),
    }
  }
}

impl Display for DecodeError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let s = match *self {
      DecodeError::InvalidRead(_) => "Error while reading",
      DecodeError::InvalidMessage(_) => "Error while decoding",
    };

    fmt.write_str(s)
  }
}

impl From<RmpvDecodeError> for Box<DecodeError> {
  fn from(err: RmpvDecodeError) -> Box<DecodeError> {
    Box::new(DecodeError::InvalidRead(err))
  }
}

impl From<RmpvDecodeError> for DecodeError {
  fn from(err: RmpvDecodeError) -> DecodeError {
    DecodeError::InvalidRead(err)
  }
}

impl From<InvalidMessageError> for Box<DecodeError> {
  fn from(err: InvalidMessageError) -> Box<DecodeError> {
    Box::new(DecodeError::InvalidMessage(err))
  }
}
