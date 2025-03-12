use derive_more::From;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
    String(std::string::String),
    Dialoguer(dialoguer::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO Error: {}", err),
            Error::Serde(err) => write!(f, "Serde Error: {}", err),
            Error::String(err) => write!(f, "String Error: {}", err),
            Error::Dialoguer(err) => write!(f, "Dialoguer Error: {}", err),
        }
    }
}
impl std::error::Error for Error {}
