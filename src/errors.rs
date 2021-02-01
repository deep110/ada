//! The `errors` module defines the common error types.

use core::fmt;

use super::Result;

/// `Error` provides an enumeration of all possible errors reported by Ada.
#[derive(Debug)]
pub enum Error {
    /// Unable to init canvas with supplied buffer
    Unsupported(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Unsupported(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Unsupported(_) => None,
        }
    }
}

/// function to create an unsupported error.
pub fn unsupported_error<T>(msg: &'static str) -> Result<T> {
    Err(Error::Unsupported(msg))
}
