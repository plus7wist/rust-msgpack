use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InternalError,
    EOFError,
    RWNotMatch,
    InvalidCode(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InternalError => f.write_str("InternalError"),
            Error::EOFError => f.write_str("EOFError"),
            Error::RWNotMatch => f.write_str("RWNotMatch"),
            Error::InvalidCode(c) => write!(f, "InvalidCode: {}", c),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::EOFError => "End Of File",
            Error::InternalError => "Internal error",
            Error::RWNotMatch => "read write not match with expected number",
            Error::InvalidCode(_) => "Invalid codes::Code",
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        match e {
            _ => Error::InternalError,
        }
    }
}
