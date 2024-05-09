use crate::protocol::{RespDecode, RespEncode, RespError};
use bytes::BytesMut;

/// :[<+|->]<value>\r\n

impl RespDecode for i64 {
    const PREFIX: &'static str = ":";
    const TYPE: &'static str = "integer";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        Self::check_format(buf)?;
        let end = super::find_crlf(buf, 1).ok_or(RespError::NotComplete)?;
        let data = buf.split_to(end + super::CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(s.parse()
            .map_err(|_| RespError::InvalidFrame(format!("Expected integer, but got: {}", s))))?
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        todo!()
    }
}

impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        let key = format!(":{}{}\r\n", sign, self);
        key.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_integer_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b":+123\r\n");

        let frame = i64::decode(&mut buf)?;
        assert_eq!(frame, 123);

        buf.extend_from_slice(b":-123\r\n");

        let frame = i64::decode(&mut buf)?;
        assert_eq!(frame, -123);
        Ok(())
    }

    #[test]
    fn test_integer_encode() {
        let i = 123;
        let data = i.encode();
        assert_eq!(data, b":+123\r\n");
    }
}
