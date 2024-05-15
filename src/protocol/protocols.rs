use crate::protocol::array::RespArray;
use crate::protocol::bulk_strings::BulkString;
use crate::protocol::map::RespMap;
use crate::protocol::null::RespNull;
use crate::protocol::set::RespSet;
use crate::protocol::simple_error::SimpleError;
use crate::protocol::simple_string::SimpleString;
use crate::protocol::{RespDecode, RespError};
use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    // NullBulkString(RespNullBulkString),
    Array(RespArray),
    // NullArray(RespNullArray),
    Null(RespNull),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}
impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";
    const TYPE: &'static str = "RespFrame";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => {
                let frame = SimpleString::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'-') => {
                let frame = SimpleError::decode(buf)?;
                Ok(frame.into())
            }
            Some(b':') => {
                let frame = i64::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'$') => {
                // try null bulk string first
                let frame = BulkString::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'*') => {
                // try null array first
                let frame = RespArray::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'_') => {
                let frame = RespNull::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'#') => {
                let frame = bool::decode(buf)?;
                Ok(frame.into())
            }
            Some(b',') => {
                let frame = f64::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'%') => {
                let frame = RespMap::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'~') => {
                let frame = RespSet::decode(buf)?;
                Ok(frame.into())
            }
            None => Err(RespError::NotComplete),
            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'~') => RespSet::expect_length(buf),
            Some(b'-') => SimpleError::expect_length(buf),
            Some(b'*') => RespArray::expect_length(buf),
            Some(b'%') => RespMap::expect_length(buf),
            Some(b'$') => BulkString::expect_length(buf),
            Some(b':') => i64::expect_length(buf),
            Some(b'+') => SimpleString::expect_length(buf),
            Some(b'#') => bool::expect_length(buf),
            Some(b',') => f64::expect_length(buf),
            Some(b'_') => RespNull::expect_length(buf),
            _ => Err(RespError::NotComplete),
        }
    }
}

impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string()).into()
    }
}

impl From<Option<&[u8]>> for RespFrame {
    fn from(value: Option<&[u8]>) -> Self {
        match value {
            Some(value) => BulkString(Some(value.to_vec())).into(),
            None => BulkString(None).into(),
        }
    }
}

impl From<&[u8]> for RespFrame {
    fn from(s: &[u8]) -> Self {
        BulkString(Some(s.to_vec())).into()
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(s: &[u8; N]) -> Self {
        BulkString(Some(s.to_vec())).into()
    }
}
