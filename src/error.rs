//! Error types
use std::{convert::From, error, fmt, io, result};

/// Error type for TPLinker
#[derive(Debug)]
pub enum Error {
    /// Wrapped errors from std::io
    IO(io::Error),
    /// Wrapped errors from serde_json
    Serde(serde_json::Error),
    /// Error decoding a section of the JSON response
    TPLink(SectionError),
    /// A generic error
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(_) => f.write_str("Error connecting to the device"),
            Error::Serde(_) => f.write_str("Could not parse the response received from the device"),
            Error::TPLink(err) => f.write_str(&format!(
                "Response data error: ({}) {}",
                err.err_code, err.err_msg
            )),
            Error::Other(err) => f.write_str(&err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::IO(_) => "Error connecting to the device",
            Error::Serde(_) => "Could not parse the response received from the device",
            Error::TPLink(_) => "Response data error",
            Error::Other(err) => err.as_str(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Serde(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::from(String::from(error))
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Other(error)
    }
}

impl From<SectionError> for Error {
    fn from(error: SectionError) -> Self {
        Error::TPLink(error)
    }
}

/// TPLinker result type with [Error](enum.Error.html)
pub type Result<T> = result::Result<T, Error>;

/// Error response for a section of the JSON response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SectionError {
    /// The error code. Zero if no error.
    pub err_code: i16,

    /// The error message.
    pub err_msg: String,
}

impl fmt::Display for SectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}: {}", self.err_code, self.err_msg))
    }
}

impl error::Error for SectionError {
    fn description(&self) -> &str {
        "TPLink section error"
    }
}
