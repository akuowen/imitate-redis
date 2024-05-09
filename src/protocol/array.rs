use bytes::BytesMut;

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
    const TYPE: &'static str = "";

    fn decode(_buf: &mut BytesMut) -> Result<Self, RespError> {
        todo!()
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        todo!()
    }
}
