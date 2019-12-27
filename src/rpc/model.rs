use crate::{
  callerror::{DecodeError, EncodeError},
  runtime::{
    AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufWriter, Mutex,
  },
};
use rmpv::{decode::read_value, encode::write_value, Value};
use std::{
  self,
  io::{self, Cursor, ErrorKind, Read},
  sync::Arc,
};

// A msgpack-rpc message, se
// https://github.com/msgpack-rpc/msgpack-rpc/blob/master/spec.md
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

macro_rules! rpc_args {
    ($($e:expr), *) => {{
        let mut vec = Vec::new();
        $(
            vec.push(Value::from($e));
        )*
        Value::from(vec)
    }}
}

// Continously reads from reader, pushing onto rest. Then tries to decode the
// contents of rest. If it succeeds, returns the message, and leaves any
// non-decoded bytes in rest. If we did not read enough for a full message, read
// more. Return on all other errors.
pub async fn decode<R: AsyncRead + Send + Unpin + 'static>(
  reader: &mut R,
  rest: &mut Vec<u8>,
) -> std::result::Result<RpcMessage, Box<DecodeError>> {
  let mut buf = Box::new([0u8; 80 * 1024]);
  let mut bytes_read;

  loop {
    let mut c = Cursor::new(&rest);

    match decode_buffer(&mut c).map_err(|b| *b) {
      Ok(msg) => {
        let pos = c.position();
        // Following cast is save since we got this from a vec index
        *rest = rest.split_off(pos as usize); // TODO: more efficiency
        return Ok(msg);
      }
      Err(DecodeError::BufferError(e))
        if e.kind() == ErrorKind::UnexpectedEof =>
      {
        debug!("Not enough data, reading more!");
        bytes_read = reader.read(&mut *buf).await;
      }
      Err(err) => return Err(err)?,
    }

    match bytes_read {
      Ok(n) if n == 0 => {
        return Err(io::Error::new(ErrorKind::UnexpectedEof, "EOF").into());
      }
      Ok(n) => {
        rest.extend_from_slice(&buf[..n]);
      }
      Err(err) => return Err(err)?,
    }
  }
}

// Syncronously decode the content of a reader into an rpc message. Tries to
// give detailed errors if something went wrong.
fn decode_buffer<R: Read>(
  reader: &mut R,
) -> std::result::Result<RpcMessage, Box<DecodeError>> {
  use crate::callerror::InvalidMessageError::*;

  let arr = match read_value(reader)? {
    Value::Array(v) => v,
    val => Err(NotAnArray(val))?,
  };

  let mut arr = arr.into_iter();

  match arr
    .next()
    .ok_or(WrongArrayLength(3..=4, 0))?
    .as_u64()
    .ok_or(InvalidMessageType)?
  {
    0 => {
      let msgid = arr
        .next()
        .ok_or(WrongArrayLength(4..=4, 1))?
        .as_u64()
        .ok_or(InvalidMsgid)?;
      let method = match arr.next() {
        Some(Value::String(s)) => {
          s.into_str().ok_or(InvalidRequestName(msgid))?
        }
        Some(_) => return Err(InvalidRequestName(msgid))?,
        None => return Err(WrongArrayLength(4..=4, 2))?,
      };
      let params = match arr.next() {
        Some(Value::Array(v)) => v,
        Some(val) => return Err(InvalidParams(val, method))?,
        None => return Err(WrongArrayLength(4..=4, 3))?,
      };

      Ok(RpcMessage::RpcRequest {
        msgid,
        method,
        params,
      })
    }
    1 => {
      let msgid = arr
        .next()
        .ok_or(WrongArrayLength(4..=4, 1))?
        .as_u64()
        .ok_or(InvalidMsgid)?;
      let error = arr.next().ok_or(WrongArrayLength(4..=4, 2))?;
      let result = arr.next().ok_or(WrongArrayLength(4..=4, 3))?;
      Ok(RpcMessage::RpcResponse {
        msgid,
        error,
        result,
      })
    }
    2 => {
      let method = match arr.next() {
        Some(Value::String(s)) => {
          s.into_str().ok_or(InvalidNotificationName)?
        }
        Some(_) => return Err(InvalidNotificationName)?,
        None => return Err(WrongArrayLength(3..=3, 1))?,
      };
      let params = match arr.next() {
        Some(Value::Array(v)) => v,
        Some(val) => return Err(InvalidParams(val, method))?,
        None => return Err(WrongArrayLength(3..=3, 2))?,
      };
      Ok(RpcMessage::RpcNotification { method, params })
    }
    t => Err(UnknownMessageType(t))?,
  }
}

// Encode the given message into the BufWriter. Flushes the writer when
// finished.
pub async fn encode<W: AsyncWrite + Send + Unpin + 'static>(
  writer: Arc<Mutex<BufWriter<W>>>,
  msg: RpcMessage,
) -> std::result::Result<(), Box<EncodeError>> {
  let mut v: Vec<u8> = vec![];
  match msg {
    RpcMessage::RpcRequest {
      msgid,
      method,
      params,
    } => {
      let val = rpc_args!(0, msgid, method, params);
      write_value(&mut v, &val)?;
    }
    RpcMessage::RpcResponse {
      msgid,
      error,
      result,
    } => {
      let val = rpc_args!(1, msgid, error, result);
      write_value(&mut v, &val)?;
    }
    RpcMessage::RpcNotification { method, params } => {
      let val = rpc_args!(2, method, params);
      write_value(&mut v, &val)?;
    }
  };

  let mut writer = writer.lock().await;
  writer.write_all(&v).await?;
  writer.flush().await?;

  Ok(())
}

pub trait FromVal<T> {
  fn from_val(_: T) -> Self;
}

impl FromVal<Value> for () {
  fn from_val(_: Value) -> Self {
    ()
  }
}

impl FromVal<Value> for Value {
  fn from_val(val: Value) -> Self {
    val
  }
}

impl FromVal<Value> for Vec<(Value, Value)> {
  fn from_val(val: Value) -> Self {
    if let Value::Map(vec) = val {
      return vec;
    }
    panic!("Not supported value for map");
  }
}

impl<T: FromVal<Value>> FromVal<Value> for Vec<T> {
  fn from_val(val: Value) -> Self {
    if let Value::Array(arr) = val {
      return arr.into_iter().map(T::from_val).collect();
    }
    panic!("Can't convert to array");
  }
}

impl FromVal<Value> for (i64, i64) {
  fn from_val(val: Value) -> Self {
    let res = val
      .as_array()
      .expect("Can't convert to point(i64,i64) value");
    if res.len() != 2 {
      panic!("Array length must be 2");
    }

    (
      res[0].as_i64().expect("Can't get i64 value at position 0"),
      res[1].as_i64().expect("Can't get i64 value at position 1"),
    )
  }
}

impl FromVal<Value> for bool {
  fn from_val(val: Value) -> Self {
    if let Value::Boolean(res) = val {
      return res;
    }
    panic!("Can't convert to bool");
  }
}

impl FromVal<Value> for String {
  fn from_val(val: Value) -> Self {
    val.as_str().expect("Can't convert to string").to_owned()
  }
}

impl FromVal<Value> for i64 {
  fn from_val(val: Value) -> Self {
    val.as_i64().expect("Can't convert to i64")
  }
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

#[cfg(test)]
mod test {
  use super::*;
  use crate::runtime::{BufWriter, Mutex};
  use std::{io::Cursor, sync::Arc};

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
    let msg = RpcMessage::RpcRequest {
      msgid: 1,
      method: "test_method".to_owned(),
      params: vec![],
    };

    let msg2 = RpcMessage::RpcRequest {
      msgid: 2,
      method: "test_method_2".to_owned(),
      params: vec![],
    };

    let buff: Vec<u8> = vec![];
    let tmp = Arc::new(Mutex::new(BufWriter::new(buff)));
    let tmp_c = tmp.clone();
    let msg_c = msg.clone();
    let msg2_c = msg2.clone();

    encode(tmp_c, msg_c).await.unwrap();
    let tmp_c = tmp.clone();
    encode(tmp_c, msg2_c).await.unwrap();
    let len = (*tmp).lock().await.get_ref().len();
    assert_eq!(34, len); // Note: msg2 is 2 longer than msg

    let v = &mut *tmp.lock().await;
    let x = v.get_mut();
    let mut cursor = Cursor::new(x.as_slice());
    let msg_dest = decode_buffer(&mut cursor).unwrap();

    assert_eq!(msg, msg_dest);
    assert_eq!(16, cursor.position());

    let msg_dest_2 = decode_buffer(&mut cursor).unwrap();
    assert_eq!(msg2, msg_dest_2);
  }
}
