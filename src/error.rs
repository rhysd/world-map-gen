use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    CannotDetermineTermsize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::CannotDetermineTermsize => write!(f, "Cannot determine terminal size"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
