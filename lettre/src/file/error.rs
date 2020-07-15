//! Error and result type for file transport

use self::Error::*;
use serde_json;
use std::io;
use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

/// An enum of all error kinds.
#[derive(Debug)]
pub enum Error {
    /// Internal client error
    Client(&'static str),
    /// IO error
    Io(io::Error),
    /// JSON serialization error
    JsonSerialization(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Client(err) => fmt.write_str(err),
            Io(ref err) => err.fmt(fmt),
            JsonSerialization(ref err) => err.fmt(fmt),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Io(ref err) => Some(&*err),
            JsonSerialization(ref err) => Some(&*err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonSerialization(err)
    }
}

impl From<&'static str> for Error {
    fn from(string: &'static str) -> Error {
        Error::Client(string)
    }
}

/// SMTP result type
pub type FileResult = Result<(), Error>;
