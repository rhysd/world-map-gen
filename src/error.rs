//! Provides error type to represent all kinds of errors which may occur while world map generations.

use serde_json;

use std::fmt;
use std::io;

/// Error type which represents all kinds of errors for world_map_gen crate.
#[derive(Debug)]
pub enum Error {
    /// IO errors occuring when reading from stdin/file or writing to stdout/stderr/file by wrapping
    /// `std::io::Error`.
    IoError(io::Error),
    /// An error raised when a board size cannot be determined automatically. Size must be set manually
    /// in the case.
    CannotDetermineTermsize,
    /// A fatal error when a generated board cannot be serialized into JSON.
    NotJsonSerializable(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::CannotDetermineTermsize => write!(f, "Cannot determine terminal size"),
            Error::NotJsonSerializable(err) => write!(f, "Cannot serialize as JSON: {}", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::NotJsonSerializable(err)
    }
}

/// Reprensents a value or an error in `world_map_gen` package. Similar to `std::io::Error` for
/// `std::io` package.
pub type Result<T> = std::result::Result<T, Error>;
