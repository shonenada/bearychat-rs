use std::fmt::{Display, Formatter, Result as FmtResult};
use std::env::VarError;
use std::error::Error as StdError;
use json::Error as JsonError;
use hyper::Error as HyperError;

#[derive(Debug)]
pub enum Error {
    // Cannot connect to server
    NetworkError(HyperError),
    // Cannot encode/decode data
    ValueError(JsonError),
}

use self::Error::{
    NetworkError,
    ValueError,
};

pub type Result<T> = ::std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f : &mut Formatter) -> FmtResult {
        match *self {
            NetworkError(ref e) => Display::fmt(e, f),
            ValueError(ref e) => Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            NetworkError(ref e) => e.description(),
            ValueError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            NetworkError(ref err) => Some(err),
            ValueError(ref err) => Some(err),
        }
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        NetworkError(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        ValueError(err)
    }
}
