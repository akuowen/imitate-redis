use crate::protocol::{RespDecode, RespEncode, RespError};
use bytes::BytesMut;

/// #<t|f>\r\n
impl RespDecode for bool {
    const PREFIX: &'static str = "#";
    const TYPE: &'static str = "bool";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if buf.len() < 4 {
            return Err(RespError::NotComplete);
        }
        Self::check_format(buf)?;
        if buf.starts_with(b"#t\r\n") {
            return Ok(true);
        }
        if buf.starts_with(b"#f\r\n") {
            return Ok(false);
        }
        Err(RespError::InvalidFrame(format!(
            "{} type false {:?}",
            Self::TYPE,
            buf
        )))
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(4)
    }
}

impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let key = format!("#{}\r\n", if self { "t" } else { "f" });
        key.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::RespFrame;
    use bytes::BytesMut;

    #[test]
    fn test_bool() {
        let mut buf = BytesMut::new();
        let value = true;
        let frame: RespFrame = value.into();
        assert_eq!(frame.encode(), b"#t\r\n");
        let encoded = value.encode();
        buf.extend_from_slice(&encoded);
        let decoded = bool::decode(&mut buf).unwrap();
        assert_eq!(value, decoded);
    }
}
