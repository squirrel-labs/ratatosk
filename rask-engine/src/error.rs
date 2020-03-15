use std::error::Error;
use std::fmt::{self, Display};

/// The error type used by the game engine.
#[derive(Debug)]
pub enum EngineError {
    ResourceError(String),
    MathError(String),
    Misc(String),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineError::ResourceError(e) => write!(f, "ResourceError: {}", e),
            EngineError::Misc(e) => write!(f, "EngineError: {}", e),
            EngineError::MathError(e) => write!(f, "MathError: {}", e),
        }
    }
}

impl Error for EngineError {}

macro_rules! derive_from {
    ($type: ty, $kind: ident) => {
        impl From<$type> for EngineError {
            fn from(error: $type) -> Self {
                EngineError::$kind(format!("{}", error))
            }
        }
    };
}

derive_from!(&str, Misc);
derive_from!(String, Misc);
derive_from!(image::error::ImageError, ResourceError);
derive_from!(stackvec::error::IncompleteArrayError, MathError);
