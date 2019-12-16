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

use rmpv::{
  decode::Error as RmpvDecodeError, encode::Error as RmpvEncodeError,
};

use std::{fmt::Display, io, ops::RangeInclusive};

use crate::runtime::oneshot;

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
  BufferReadError(RmpvDecodeError),
  ReaderError(io::Error),
  InvalidMessage(InvalidMessageError),
}

impl Error for DecodeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      DecodeError::BufferReadError(ref e) => Some(e),
      DecodeError::InvalidMessage(ref e) => Some(e),
      DecodeError::ReaderError(ref e) => Some(e),
    }
  }
}

impl Display for DecodeError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let s = match *self {
      DecodeError::BufferReadError(_) => "Error while reading from buffer",
      DecodeError::InvalidMessage(_) => "Error while decoding",
      DecodeError::ReaderError(_) => "Error while reading from Reader",
    };

    fmt.write_str(s)
  }
}

impl From<RmpvDecodeError> for Box<DecodeError> {
  fn from(err: RmpvDecodeError) -> Box<DecodeError> {
    Box::new(DecodeError::BufferReadError(err))
  }
}

impl From<RmpvDecodeError> for DecodeError {
  fn from(err: RmpvDecodeError) -> DecodeError {
    Self::BufferReadError(err)
  }
}

impl From<InvalidMessageError> for Box<DecodeError> {
  fn from(err: InvalidMessageError) -> Box<DecodeError> {
    Box::new(DecodeError::InvalidMessage(err))
  }
}

impl From<io::Error> for DecodeError {
  fn from(err: io::Error) -> DecodeError {
    Self::ReaderError(err)
  }
}

impl From<io::Error> for Box<DecodeError> {
  fn from(err: io::Error) -> Box<DecodeError> {
    Box::new(DecodeError::ReaderError(err))
  }
}

#[derive(Debug)]
pub enum EncodeError {
  BufferWriteError(RmpvEncodeError),
  WriterError(io::Error),
}

impl Error for EncodeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      EncodeError::BufferWriteError(ref e) => Some(e),
      EncodeError::WriterError(ref e) => Some(e),
    }
  }
}

impl Display for EncodeError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let s = match *self {
      Self::BufferWriteError(_) => "Error writing to buffer",
      Self::WriterError(_) => "Error writing to the Writer",
    };

    fmt.write_str(s)
  }
}

impl From<RmpvEncodeError> for EncodeError {
  fn from(err: RmpvEncodeError) -> EncodeError {
    Self::BufferWriteError(err)
  }
}

impl From<RmpvEncodeError> for Box<EncodeError> {
  fn from(err: RmpvEncodeError) -> Box<EncodeError> {
    Box::new(EncodeError::BufferWriteError(err))
  }
}

impl From<io::Error> for EncodeError {
  fn from(err: io::Error) -> EncodeError {
    Self::WriterError(err)
  }
}

impl From<io::Error> for Box<EncodeError> {
  fn from(err: io::Error) -> Box<EncodeError> {
    Box::new(EncodeError::WriterError(err))
  }
}

#[derive(Debug)]
pub enum CallError2 {
  SendError(EncodeError, String),
  ReceiveError(oneshot::error::RecvError, String),
  OldCallError(CallError)
}

impl Error for CallError2 {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      CallError2::SendError(ref e, _) => Some(e),
      CallError2::ReceiveError(ref e, _) => Some(e),
      CallError2::OldCallError(ref e) => Some(e),
    }
  }
}

impl Display for CallError2 {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Self::SendError(_, ref s) => write!(fmt, "Error sending request '{}'", s),
      Self::ReceiveError(_, ref s) => {
        write!(fmt, "Error receiving response for '{}'", s)
      }
      Self::OldCallError(ref err) => write!(fmt, "{}", err),
    }
  }
}

impl From<CallError> for CallError2 {
  fn from(err: CallError) -> CallError2 {
    CallError2::OldCallError(err)
  }
}

impl From<CallError> for Box<CallError2> {
  fn from(err: CallError) -> Box<CallError2> {
    Box::new(CallError2::OldCallError(err))
  }
}
