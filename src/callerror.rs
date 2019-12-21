use rmpv::{
  decode::Error as RmpvDecodeError, encode::Error as RmpvEncodeError, Value,
};
use std::{error::Error, fmt};
use std::io::ErrorKind;
use std::sync::Arc;

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

/// An error to communicate the failure to decode a message from neovim.
#[derive(Debug)]
pub enum DecodeError {
  /// Reading from the internal buffer failed. This is recovered from by trying
  /// to read from the stream again and should never surface to the user.
  ///
  /// **TODO**: Can we remove this from the public interface?
  BufferReadError(RmpvDecodeError),
  /// Reading from the stream failed. This is probably unrecoverable from, but
  /// might also mean that neovim closed the stream and wants the plugin to
  /// finish. See examples/quitting.rs on how this might be caught.
  ReaderError(io::Error),
  /// Neovim sent a message that's not valid.
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

/// An error to communicate that sending a message to neovim has failed.
#[derive(Debug)]
pub enum EncodeError {
  /// Encoding the message into the internal buffer has failed.
  BufferWriteError(RmpvEncodeError),
  /// Writing the encoded message to the stream failed.
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

/// Error to communicate the failure of a [`call`](crate::neovim::Neovim::call) to
/// neovim. The API functions return this, as they are just
/// proxies for [`call`](crate::neovim::Neovim::call).
#[derive(Debug)]
pub enum CallError {
  /// Sending the request to neovim has failed.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The name of the called method 
  SendError(EncodeError, String),
  /// The internal channel to send the response to the right task was closed.
  /// This really should not happen, unless someone manages to kill individual
  /// tasks.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The name of the called method 
  ReceiveError(oneshot::error::RecvError, String),
  /// Decoding neovim's response failed.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The name of the called method 
  ///
  /// *Note*: DecodeError can't be Clone, so we Arc-wrap it
  DecodeError(Arc<DecodeError>, String),
  /// Neovim encountered an error while executing the reqest.
  ///
  /// Fields:
  ///
  /// 0. Neovim's error type (see `:h api`)
  /// 1. Neovim's error message
  NeovimError(Option<i64>, String),
}

impl Error for CallError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      CallError::SendError(ref e, _) => Some(e),
      CallError::ReceiveError(ref e, _) => Some(e),
      CallError::DecodeError(ref e, _) => Some(e.as_ref()),
      CallError::NeovimError(_, _) => None,
    }
  }
}

impl CallError {
  pub fn is_channel_closed(&self) -> bool {
    match *self {
      CallError::SendError(EncodeError::WriterError(ref e), _) if e.kind() ==
        ErrorKind::UnexpectedEof => {
          return true
      }
      CallError::DecodeError(ref err, _) => {
         if let DecodeError::ReaderError(ref e) = err.as_ref() {
           if e.kind() == ErrorKind::UnexpectedEof {
             return true
           }
         }
      }
      _ => {}
    }

    return false;
  }
}

impl Display for CallError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Self::SendError(_, ref s) => write!(fmt, "Error sending request '{}'", s),
      Self::ReceiveError(_, ref s) => {
        write!(fmt, "Error receiving response for '{}'", s)
      }
      Self::DecodeError(_, ref s) => write!(fmt, "Error decoding response to request '{}'", s),
      Self::NeovimError(ref i, ref s) => match i {
        Some(i) => write!(fmt, "Error processing request: {} - '{}')", i, s),
        None => write!(
          fmt,
          "Error processing request, unknown error format:
            '{}'",
          s
        ),
      },
    }
  }
}

impl From<Value> for CallError {
  fn from(val: Value) -> CallError {
    match val {
      Value::Array(mut arr)
        if arr.len() == 2 && arr[0].is_i64() && arr[1].is_str() =>
      {
        let s = arr.pop().unwrap().as_str().unwrap().into();
        let i = arr.pop().unwrap().as_i64();
        CallError::NeovimError(i, s)
      }
      val => CallError::NeovimError(None, format!("{:?}", val)),
    }
  }
}

impl From<Value> for Box<CallError> {
  fn from(val: Value) -> Box<CallError> {
    match val {
      Value::Array(mut arr)
        if arr.len() == 2 && arr[0].is_i64() && arr[1].is_str() =>
      {
        let s = arr.pop().unwrap().as_str().unwrap().into();
        let i = arr.pop().unwrap().as_i64();
        Box::new(CallError::NeovimError(i, s))
      }
      val => Box::new(CallError::NeovimError(None, format!("{:?}", val))),
    }
  }
}

#[derive(Debug)]
pub enum LoopError {
  /// A Msgid could not be found in the Queue
  MsgidNotFoundError(u64),
  /// Could not send an error to all callers in the Queue. Contains the msgids
  /// of the waiting requests as well as the error to send
  /// Note: DecodeError can't be clone, so we Arc-wrap it.
  SendToCallersError(Vec<u64>, Arc<DecodeError>),
  /// Failed to send a Response (from neovim) through the sender from the Queue
  SendResponseError(u64, Result<Value, Value>),
  /// Sending a response to neovim failed
  EncodeError(EncodeError),
}

impl Error for LoopError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      LoopError::MsgidNotFoundError(_) => None,
      LoopError::SendToCallersError(_, ref e) => Some(e.as_ref()),
      LoopError::SendResponseError(_, _) => None,
      LoopError::EncodeError(ref e) => Some(e),
    }
  }
}

impl Display for LoopError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Self::MsgidNotFoundError(i) => write!(
        fmt,
        "Could not find Msgid '{}' in
        the Qeue",
        i
      ),
      Self::SendToCallersError(ref v, _) => write!(
        fmt,
        "Could not send responses to their callers: '{:?}'",
        v
      ),
      Self::SendResponseError(i, ref res) => write!(
        fmt,
        "Request {}: Could not send response, which was {:?}", 
        i,
        res
      ),
      Self::EncodeError(_) => write!(fmt, "Error encoding response"),
    }
  }
}

impl From<(u64, Result<Value, Value>)> for Box<LoopError> {
  fn from(res: (u64, Result<Value, Value>)) -> Box<LoopError> {
    Box::new(LoopError::SendResponseError(res.0, res.1))
  }
}

impl From<(Vec<u64>, Arc<DecodeError>)> for Box<LoopError> {
  fn from(v: (Vec<u64>, Arc<DecodeError>)) -> Box<LoopError> {
    Box::new(LoopError::SendToCallersError(v.0, v.1))
  }
}

impl From<u64> for Box<LoopError> {
  fn from(i: u64) -> Box<LoopError> {
    Box::new(LoopError::MsgidNotFoundError(i))
  }
}

impl From<Box<EncodeError>> for Box<LoopError> {
  fn from(e: Box<EncodeError>) -> Box<LoopError> {
    Box::new(LoopError::EncodeError(*e))
  }
}
