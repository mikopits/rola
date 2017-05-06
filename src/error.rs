use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Supplied matrix has invalid dimensions.
    InvalidDimensions
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidDimensions => f.pad("Supplied matrix has invalid dimensions")
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidDimensions => "Supplied matrix has invalid dimensions"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::InvalidDimensions => None
        }
    }
}

pub type Result<T> = StdResult<T, Error>;
