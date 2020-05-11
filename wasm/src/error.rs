use std::fmt;

/// Collection of frontend errors
/// these can result form Network errors, other javascript errors or concurrency errors
///
/// they implement Display
/// # Examples
///
/// ```should_panic
/// use crate::error::ClientError;
///
/// # fn main() -> Result<(), ClientError> {
///   return Err(ClientError::EngineError(format!("EngineError"));
/// # }
/// ```
#[derive(Debug)]
pub enum ClientError {
    WebGlError(String),
    ResourceError(String),
    EngineError(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::ResourceError(e)
            | ClientError::WebGlError(e)
            | ClientError::EngineError(e) => write!(f, "{}", e),
        }
    }
}
macro_rules! derive_from {
    ($type:ty, $kind:ident) => {
        impl From<$type> for ClientError {
            fn from(error: $type) -> Self {
                ClientError::$kind(format!("{}", error))
            }
        }
    };
}

derive_from!(rask_engine::error::EngineError, EngineError);
