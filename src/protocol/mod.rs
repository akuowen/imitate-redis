mod array;
mod protocols;

use bytes::BytesMut;
use thiserror::Error;

/*const BUF_CAP: usize = 4096;
const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();*/

pub use protocols::RespFrame;

// #[enum_dispatch]
pub trait RespEncode {
    /// encode the response to a byte array
    fn encode(self) -> Vec<u8>;
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    // #[error("Invalid frame: {0}")]
    // InvalidFrame(String),
    // #[error("Invalid frame type: {0}")]
    // InvalidFrameType(String),
    // #[error("Invalid frame length： {0}")]
    // InvalidFrameLength(isize),
    // #[error("Frame is not complete")]
    // NotComplete,
    //
    // #[error("Parse error: {0}")]
    // ParseIntError(#[from] std::num::ParseIntError),
    // #[error("Utf8 error: {0}")]
    // Utf8Error(#[from] std::string::FromUtf8Error),
    // #[error("Parse float error: {0}")]
    // ParseFloatError(#[from] std::num::ParseFloatError),
}

/// decode the response from a byte array
pub trait RespDecode: Sized {
    /// redis protocol prefix
    /// see https://redis.io/topics/protocol
    const PREFIX: &'static str;

    /// decode the response from a byte array
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;

    /// expect the length of the response
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}
