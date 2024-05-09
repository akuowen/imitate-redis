use crate::protocol::{RespDecode, RespError};
use bytes::BytesMut;

/// ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n

impl RespDecode for f64 {
    const PREFIX: &'static str = ",";
    const TYPE: &'static str = "double";

    fn decode(_buf: &mut BytesMut) -> Result<Self, RespError> {
        todo!()
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        todo!()
    }
}
