use crate::protocol::{RespDecode, RespEncode, RespError};
use bytes::BytesMut;

/// ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n

impl RespDecode for f64 {
    const PREFIX: &'static str = ",";
    const TYPE: &'static str = "double";

    /// decode f64 from RESP
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        // CRLF start
        let end = super::extract_simple_frame_data(buf, Self::PREFIX)?;
        // data
        let data = buf.split_to(end + super::CRLF_LEN);
        // data body
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(s.parse()
            .map_err(|_| RespError::InvalidFrame(format!("Expected integer, but got: {}", s))))?
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = super::extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + super::CRLF_LEN)
    }
}

impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };

        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{RespDecode, RespEncode, RespFrame};

    #[test]
    fn test_f64() {
        let s = 3.15;
        let encoded = s.encode();
        assert_eq!(encoded, b",+3.15\r\n");
        let mut data = BytesMut::new();
        data.extend_from_slice(b",+3.15\r\n");
        let decoded = f64::decode(&mut data).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn test_f64_negative() {
        let s: f64 = -3.15;
        let encoded = s.encode();
        assert_eq!(encoded, b",-3.15\r\n");
        let mut data = BytesMut::new();
        data.extend_from_slice(b",-3.15\r\n");
        let decoded = f64::decode(&mut data).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn test_f64_large() {
        let frame: RespFrame = 1.23456e+8.into();
        assert_eq!(frame.encode(), b",+1.23456e8\r\n");
    }

    #[test]
    fn test_f64_small() {
        let s = 0.00000001;
        let encoded = s.encode();
        let result = String::from_utf8(encoded).unwrap();
        assert_eq!(result.as_bytes(), b",+0.00000001\r\n");
        let mut data = BytesMut::new();
        data.extend_from_slice(b",+0.00000001\r\n");
        let decoded = f64::decode(&mut data).unwrap();
        assert_eq!(decoded, s);
    }
}
