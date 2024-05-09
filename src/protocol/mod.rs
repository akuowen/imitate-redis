mod array;
mod bool;
mod bulk_strings;
mod double;
mod integer;
mod protocols;

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use thiserror::Error;

use self::array::RespArray;
pub use protocols::RespFrame;
const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

#[enum_dispatch]
pub trait RespEncode {
    /// encode the response to a byte array
    fn encode(self) -> Vec<u8>;
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    // #[error("Invalid frame length： {0}")]
    // InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,
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
    const TYPE: &'static str;

    /// decode the response from a byte array
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;

    fn check_format(buf: &mut BytesMut) -> Result<(), RespError> {
        buf.starts_with(Self::PREFIX.as_bytes())
            .then_some(())
            .ok_or_else(|| {
                RespError::InvalidFrameType(format!(
                    "Expected prefix: {}, but got: {:?}",
                    Self::TYPE,
                    buf
                ))
            })
    }

    /// expect the length of the response
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

/// find the nth CRLF in the buffer
fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    let mut count = 0;
    for i in 1..buf.len() - 1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            count += 1;
            if count == nth {
                return Some(i);
            }
        }
    }
    None
}

/// parse the length of the response from the buffer (end,length)
fn parse_length(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespError> {
    let end = extract_simple_frame_data(buf, prefix)?;
    let s = String::from_utf8_lossy(&buf[prefix.len()..end]);
    Ok((
        end,
        s.parse()
            .map_err(|_| RespError::InvalidFrame(format!("Invalid length: {:?}", s)))?,
    ))
}

fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: SimpleString({}), got: {:?}",
            prefix, buf
        )));
    }

    let end = find_crlf(buf, 1).ok_or(RespError::NotComplete)?;

    Ok(end)
}
