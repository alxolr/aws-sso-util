use derive_more::From;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(std::io::Error),
    #[error("Serde error: {0}")]
    Serde(serde_json::Error),
    #[error("Key not found")]
    KeyNotFound,
    #[error("Value not found")]
    ValueNotFound,
    #[error("Dialoguer error: {0}")]
    Dialoguer(dialoguer::Error),
    #[error("String error: {0}")]
    Str(String),
}
