use std::error::Error;
use std::fmt::{self, Display};

/// The error type used by the game engine.
#[derive(Debug)]
pub enum EngineError {
    ResourceError(String),
    Misc(String),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineError::ResourceError(e) => write!(f, "ResourceError: {}", e),
            EngineError::Misc(e) => write!(f, "EngineError: {}", e),
        }
    }
}

impl Error for EngineError {}

impl From<&str> for EngineError {
    fn from(error: &str) -> Self {
        EngineError::Misc(error.to_owned())
    }
}
impl From<String> for EngineError {
    fn from(error: String) -> Self {
        EngineError::Misc(error)
    }
}
impl From<image::error::ImageError> for EngineError {
    fn from(error: image::error::ImageError) -> Self {
        EngineError::ResourceError(format!("{}", error))
    }
}
