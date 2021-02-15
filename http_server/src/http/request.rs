use super::method::Method;
use std::str;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    // super goes one level up in the module hierarchy
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}


// fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

    // valid
//     match str::from_utf8(buf) {
//         Ok(request) => {}
//         Err(_) => return Err(ParseError::InvalidEncoding)
//     }

    // valid
//     match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
//         Ok(request) => {}
//         Err(e) => return Err(e)
//     }

    // idiomatic Rust that does the same as the above match stmts
//     let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
// }
