use serde::ser;
use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("RON parse error: {0}")]
    RonParse(#[from] ron::error::SpannedError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
