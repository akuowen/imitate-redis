use bytes::{Buf, BytesMut};
use std::ops::Deref;

use super::{RespDecode, RespEncode, RespError, RespFrame};

const NULL: &str = "*-1\r\n";
/// *<number-of-elements>\r\n<element-1>...<element-n>
///

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespArray(pub(crate) Option<Vec<RespFrame>>);

impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        match self.0 {
            Some(frame) => {
                let mut bytes = Vec::with_capacity(1024);
                // * number-of-elements \r\n
                bytes.extend_from_slice(&format!("*{}\r\n", frame.len()).into_bytes());
                // <element-1>...<element-n>
                frame.into_iter().for_each(|f| {
                    bytes.extend_from_slice(&f.encode());
                });
                bytes
            }
            None => b"*-1\r\n".to_vec(),
        }
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";
    const TYPE: &'static str = "array";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let is_null = super::extract_fixed_data_noadv(buf, NULL, "NullArray");
        match is_null {
            Ok(_) => {
                buf.advance(NULL.len());
                return Ok(RespArray::new_null());
            }
            Err(RespError::NotComplete) => return Err(RespError::NotComplete),
            _ => {}
        }
        let (end, len) = super::parse_length(buf, Self::PREFIX)?;
        let total_len = super::calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + super::CRLF_LEN);

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }

        Ok(RespArray::new(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = super::parse_length(buf, Self::PREFIX)?;
        super::calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(Some(s.into()))
    }

    pub fn new_null() -> Self {
        RespArray(None)
    }
}

impl Deref for RespArray {
    type Target = Option<Vec<RespFrame>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Option<Vec<RespFrame>>> for RespArray {
    fn from(s: Option<Vec<RespFrame>>) -> Self {
        RespArray(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::BulkString;
    use anyhow::Result;
    #[test]
    fn test_array_encode() {
        let frame: RespFrame = RespArray::new(vec![
            BulkString::new("set".to_string()).into(),
            BulkString::new("hello".to_string()).into(),
            BulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            &frame.encode(),
            b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let frame: RespArray = None.into();
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");

        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new_null());

        Ok(())
    }

    #[test]
    fn test_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));

        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n");
        let ret = RespArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.extend_from_slice(b"$5\r\nhello\r\n");
        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));

        Ok(())
    }
}
