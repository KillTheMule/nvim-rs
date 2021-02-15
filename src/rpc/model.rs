//! Decoding and encoding msgpack rpc messages from/to neovim.
use std::{
  self,
  convert::TryInto,
  fmt,
  io::{self, Cursor, ErrorKind, Read},
  sync::Arc,
};

use futures::{
  io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufWriter},
  lock::Mutex,
};
use rmp_serde::encode;
use rmpv::ext::from_value;
use rmpv::{decode::read_value, encode::write_value, Value};
use serde::ser::{SerializeSeq, SerializeTuple};
use serde::{
  de::{self, SeqAccess, Visitor},
  Deserialize, Deserializer, Serialize, Serializer,
};

use crate::error::{DecodeError, EncodeError};

/// A msgpack-rpc message, see
/// <https://github.com/msgpack-rpc/msgpack-rpc/blob/master/spec.md>
#[derive(Debug, PartialEq, Clone)]
pub enum RpcMessage {
  RpcRequest {
    msgid: u64,
    method: String,
    params: Vec<Value>,
  }, // 0
  RpcResponse {
    msgid: u64,
    error: Value,
    result: Value,
  }, // 1
  RpcNotification {
    method: String,
    params: Vec<Value>,
  }, // 2
}

impl Serialize for RpcMessage {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      RpcMessage::RpcRequest {
        msgid,
        method,
        params,
      } => {
        let mut seq = serializer.serialize_seq(Some(4))?;
        seq.serialize_element(&0)?;
        seq.serialize_element(msgid)?;
        seq.serialize_element(method)?;
        seq.serialize_element(params)?;
        seq.end()
      }
      RpcMessage::RpcResponse {
        msgid,
        error,
        result,
      } => {
        let mut seq = serializer.serialize_seq(Some(4))?;
        seq.serialize_element(&1)?;
        seq.serialize_element(msgid)?;
        seq.serialize_element(error)?;
        seq.serialize_element(result)?;
        seq.end()
      }

      RpcMessage::RpcNotification { method, params } => {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&2)?;
        seq.serialize_element(method)?;
        seq.serialize_element(params)?;
        seq.end()
      }
    }
  }
}

impl<'de> Deserialize<'de> for RpcMessage {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct RpcVisitor;

    impl<'de> Visitor<'de> for RpcVisitor {
      type Value = RpcMessage;

      fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an array")
      }

      fn visit_seq<V>(self, mut seq: V) -> Result<RpcMessage, V::Error>
      where
        V: SeqAccess<'de>,
      {
        let res = match seq
          .next_element()?
          .ok_or_else(|| de::Error::invalid_length(0, &self))?
        {
          0 => RpcMessage::RpcRequest {
            msgid: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(1, &self))?,
            method: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(2, &self))?,
            params: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(3, &self))?,
          },
          1 => RpcMessage::RpcResponse {
            msgid: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(1, &self))?,
            error: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(2, &self))?,
            result: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(3, &self))?,
          },
          2 => RpcMessage::RpcNotification {
            method: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(1, &self))?,
            params: seq
              .next_element()?
              .ok_or_else(|| de::Error::invalid_length(2, &self))?,
          },
          i => return Err(de::Error::custom(format!("invalid id: {}", i))),
        };

        Ok(res)
      }
    }

    deserializer.deserialize_seq(RpcVisitor)
  }
}
macro_rules! rpc_args {
    ($($e:expr), *) => {{
        let mut vec = Vec::new();
        $(
            vec.push(Value::from($e));
        )*
        Value::from(vec)
    }}
}

/// Continously reads from reader, pushing onto `rest`. Then tries to decode the
/// contents of `rest`. If it succeeds, returns the message, and leaves any
/// non-decoded bytes in `rest`. If we did not read enough for a full message,
/// read more. Return on all other errors.
//
// TODO: This might be inefficient. Can't we read into `rest` directly?
pub async fn decode<R: AsyncRead + Send + Unpin + 'static>(
  reader: &mut R,
  rest: &mut Vec<u8>,
) -> std::result::Result<RpcMessage, Box<DecodeError>> {
  let mut buf = Box::new([0_u8; 80 * 1024]);
  let mut bytes_read;

  loop {
    let mut c = Cursor::new(&rest);

    match decode_buffer(&mut c).map_err(|b| *b) {
      Ok(msg) => {
        let pos = c.position();
        *rest = rest.split_off(pos as usize); // TODO: more efficiency
        return Ok(msg);
      }
      Err(DecodeError::BufferError(e))
        if e.kind() == ErrorKind::UnexpectedEof =>
      {
        debug!("Not enough data, reading more!");
        bytes_read = reader.read(&mut *buf).await;
      }
      Err(DecodeError::SerdeBufferError(
        rmp_serde::decode::Error::InvalidMarkerRead(e),
      )) if e.kind() == ErrorKind::UnexpectedEof => {
        debug!("Not enough data, reading more!");
        bytes_read = reader.read(&mut *buf).await;
      }
      Err(err) => return Err(err.into()),
    }

    match bytes_read {
      Ok(n) if n == 0 => {
        return Err(io::Error::new(ErrorKind::UnexpectedEof, "EOF").into());
      }
      Ok(n) => {
        rest.extend_from_slice(&buf[..n]);
      }
      Err(err) => return Err(err.into()),
    }
  }
}

/// Syncronously decode the content of a reader into an rpc message. Tries to
/// give detailed errors if something went wrong.
fn decode_buffer<R: Read>(
  reader: &mut R,
) -> std::result::Result<RpcMessage, Box<DecodeError>> {
  Ok(rmp_serde::decode::from_read(reader)?)
}

/// Encode the given message into the `BufWriter`. Flushes the writer when
/// finished.
pub async fn encode<W: AsyncWrite + Send + Unpin + 'static>(
  writer: Arc<Mutex<BufWriter<W>>>,
  msg: RpcMessage,
) -> std::result::Result<(), Box<EncodeError>> {
  let mut buf: Vec<u8> = vec![];
  encode::write(&mut buf, &msg)?;
  let mut writer = writer.lock().await;
  writer.write_all(&buf).await?;
  writer.flush().await?;

  Ok(())
}

pub trait IntoVal<T> {
  fn into_val(self) -> T;
}

impl<'a> IntoVal<Value> for &'a str {
  fn into_val(self) -> Value {
    Value::from(self)
  }
}

impl IntoVal<Value> for Vec<String> {
  fn into_val(self) -> Value {
    let vec: Vec<Value> = self.into_iter().map(Value::from).collect();
    Value::from(vec)
  }
}

impl IntoVal<Value> for Vec<Value> {
  fn into_val(self) -> Value {
    Value::from(self)
  }
}

impl IntoVal<Value> for (i64, i64) {
  fn into_val(self) -> Value {
    Value::from(vec![Value::from(self.0), Value::from(self.1)])
  }
}

impl IntoVal<Value> for bool {
  fn into_val(self) -> Value {
    Value::from(self)
  }
}

impl IntoVal<Value> for i64 {
  fn into_val(self) -> Value {
    Value::from(self)
  }
}

impl IntoVal<Value> for f64 {
  fn into_val(self) -> Value {
    Value::from(self)
  }
}

impl IntoVal<Value> for String {
  fn into_val(self) -> Value {
    Value::from(self)
  }
}

impl IntoVal<Value> for Value {
  fn into_val(self) -> Value {
    self
  }
}

impl IntoVal<Value> for Vec<(Value, Value)> {
  fn into_val(self) -> Value {
    Value::from(self)
  }
}

#[cfg(all(test, feature = "use_tokio"))]
mod test {
  use super::*;
  use futures::{io::BufWriter, lock::Mutex};
  use std::{io::Cursor, sync::Arc};

  use tokio;

  #[tokio::test]
  async fn request_test() {
    let msg = RpcMessage::RpcRequest {
      msgid: 1,
      method: "test_method".to_owned(),
      params: vec![],
    };

    let buff: Vec<u8> = vec![];
    let tmp = Arc::new(Mutex::new(BufWriter::new(buff)));
    let tmp2 = tmp.clone();
    let msg2 = msg.clone();

    encode(tmp2, msg2).await.unwrap();

    let msg_dest = {
      let v = &mut *tmp.lock().await;
      let x = v.get_mut();
      decode_buffer(&mut x.as_slice()).unwrap()
    };

    assert_eq!(msg, msg_dest);
  }

  #[tokio::test]
  async fn request_test_twice() {
    let msg_1 = RpcMessage::RpcRequest {
      msgid: 1,
      method: "test_method".to_owned(),
      params: vec![],
    };

    let msg_2 = RpcMessage::RpcRequest {
      msgid: 2,
      method: "test_method_2".to_owned(),
      params: vec![],
    };

    let buff: Vec<u8> = vec![];
    let tmp = Arc::new(Mutex::new(BufWriter::new(buff)));
    let msg_1_c = msg_1.clone();
    let msg_2_c = msg_2.clone();

    let tmp_c = tmp.clone();
    encode(tmp_c, msg_1_c).await.unwrap();
    let tmp_c = tmp.clone();
    encode(tmp_c, msg_2_c).await.unwrap();
    let len = (*tmp).lock().await.get_ref().len();
    assert_eq!(34, len); // Note: msg2 is 2 longer than msg

    let v = &mut *tmp.lock().await;
    let x = v.get_mut();
    let mut cursor = Cursor::new(x.as_slice());
    let msg_dest_1 = decode_buffer(&mut cursor).unwrap();

    assert_eq!(msg_1, msg_dest_1);
    assert_eq!(16, cursor.position());

    let msg_dest_2 = decode_buffer(&mut cursor).unwrap();
    assert_eq!(msg_2, msg_dest_2);
  }
}
