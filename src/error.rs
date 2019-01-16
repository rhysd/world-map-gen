//! Provides error type to represent all kinds of errors which may occur while world map generations.

use std::fmt;
use std::io;

/// Error type which represents all kinds of errors for world_map_gen crate.
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
