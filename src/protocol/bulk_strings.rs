use crate::protocol::{RespDecode, RespEncode, RespError};
use bytes::{Buf, BytesMut};
use std::ops::Deref;

const NULL: &str = "$-1\r\n";
/// $<length>\r\n<data>\r\n
/// $-1\r\n
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct BulkString(pub(crate) Option<Vec<u8>>);

impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";
    const TYPE: &'static str = "BulkString";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let is_null = super::extract_fixed_data_noadv(buf, NULL, "NullBulkString");
        match is_null {
            Ok(_) => {
                buf.advance(NULL.len());
                return Ok(BulkString::new_null());
            }
            Err(RespError::NotComplete) => return Err(RespError::NotComplete),
            _ => {}
        }
        let (end, len) = super::parse_length(buf, Self::PREFIX)?;
        let remained = &buf[end + super::CRLF_LEN..];
        if remained.len() < len + super::CRLF_LEN {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + super::CRLF_LEN);

        let data = buf.split_to(len + super::CRLF_LEN);
        Ok(BulkString::new(data[..len].to_vec()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = super::parse_length(buf, Self::PREFIX)?;
        Ok(end + super::CRLF_LEN + len + super::CRLF_LEN)
    }
}

impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let vec = self.0;
        match vec {
            Some(bytes) => {
                let length = bytes.len();

                let str = String::from_utf8_lossy(bytes.as_slice());
                let encode = format!("${}\r\n{}\r\n", length, str);
                encode.into_bytes()
            }
            None => b"$-1\r\n".to_vec(),
        }
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(Some(s.into()))
    }

    pub fn new_null() -> Self {
        BulkString(None)
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        match &self.0 {
            Some(k) => k.as_slice(),
            None => b"",
        }
    }
}

impl Deref for BulkString {
    type Target = Option<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString(Some(s.as_bytes().to_vec()))
    }
}

impl From<String> for BulkString {
    fn from(s: String) -> Self {
        BulkString(Some(s.into_bytes()))
    }
}

impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString(Some(s.to_vec()))
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString(Some(s.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::RespFrame;
    use bytes::BytesMut;

    #[test]
    fn test_bulk_string_decode() {
        let mut buf = BytesMut::from("$6\r\nfoobar\r\n");
        let result = BulkString::decode(&mut buf).unwrap();
        assert_eq!(result, BulkString::new(b"foobar".to_vec()));
    }

    #[test]
    fn test_bulk_string_encode() {
        let bulk_string = BulkString::new(b"foobar".to_vec());
        let result = bulk_string.encode();
        assert_eq!(result, b"$6\r\nfoobar\r\n");
    }

    #[test]
    fn test_bulk_string_expect_length() {
        let buf = b"$6\r\nfoobar\r\n";
        let result = BulkString::expect_length(buf).unwrap();
        assert_eq!(result, buf.len());
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame: RespFrame = None.into();
        assert_eq!(frame.encode(), b"$-1\r\n");
    }
}
