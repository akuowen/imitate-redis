use bytes::{Buf, BytesMut};
use std::ops::Deref;

use super::{RespDecode, RespEncode, RespError, RespFrame};

/// *<number-of-elements>\r\n<element-1>...<element-n>
///

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1024);
        // * number-of-elements \r\n
        bytes.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());
        // <element-1>...<element-n>
        self.0.into_iter().for_each(|frame| {
            bytes.extend_from_slice(&frame.encode());
        });
        bytes
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";
    const TYPE: &'static str = "array";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
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
        RespArray(s.into())
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
