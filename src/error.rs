use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {

    /// Error passed from std::io
    #[error("System error: {0}")]
    IOError(std::io::Error),

    #[error("Error when parsing string: {0}")]
    StringUtf8Error(std::string::FromUtf8Error),

    /// Error called when the file isn't an MSBT file
    #[error("File given is not an MSBT file")]
    NotMSBT,

    /// Error called when there's an error in the file
    #[error("File given is malformed!")]
    MalformedFile,

    /// Error called when an escape code is malformed
    #[error("Escape code is broken!")]
    MalformedEscape,

    /// Error called when serde_json
    #[error("Error when parsing JSON!")]
    MalformedJson(serde_json::Error)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::StringUtf8Error(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::MalformedJson(err)
    }
}