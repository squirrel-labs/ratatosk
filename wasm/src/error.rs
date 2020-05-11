use std::fmt;

/// Collection of frontend errors
/// these can result form Network errors, other javascript errors or concurrency errors
///
/// they implement Display
/// # Examples
///
/// ```should_panic
/// use wasm_bindgen::JsValue;
/// use crate::error::ClientError;
///
/// # fn main() -> Result<(), ClientError> {
/// let err: Result<(), JsValue> = Err(JsValue::from_str("test error"));
/// if let Err(x) = err {
///    return Err(ClientError::WebSocketError(x));
/// }
/// Ok(())
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
