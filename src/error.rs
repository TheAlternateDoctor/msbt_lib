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

    /// Error called when a toml file is malformed
    #[error("Toml file is malformed!!")]
    MalformedToml(toml::de::Error),

    /// Error called when the MSBT passed in argument doesn't match the one in the diff
    #[error("The hashes of the MSBT file and the diff file don't match!")]
    BadHash,

    /// Error called when an error happens when reading a
    #[error("Unrecognized state for a string! States should be '+' (added), '-' (deleted) or '~' (edited)!")]
    MalformedDiffUnrecognizedState,
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

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::MalformedToml(err)
    }
}
