use crate::protocol::array::RespArray;
use crate::protocol::RespEncode;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    // SimpleString(SimpleString),
    // Error(SimpleError),
    // Integer(i64),
    // BulkString(BulkString),
    // NullBulkString(RespNullBulkString),
    Array(RespArray),
    // NullArray(RespNullArray),
    // Null(RespNull),
    // Boolean(bool),
    // Double(f64),
    // Map(RespMap),
    // Set(RespSet),
}

impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        // todo!()
        match self {
            RespFrame::Array(array) => array.encode(),
        }
    }
}
