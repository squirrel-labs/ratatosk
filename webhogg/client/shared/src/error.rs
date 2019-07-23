use std::fmt;
use wasm_bindgen::JsValue;

/// Collection of frontend errors
/// these can result form Network errors, other javascript errors or concurrency errors
///
/// they implement Display
/// # Examples
///
/// ```should_panic
/// use wasm_bindgen::JsValue;
/// use webhogg_wasm_shared::error::ClientError;
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
    WebSocketError(JsValue),
}

fn jsvalue_to_string(v: &JsValue) -> String {
    // try to parse JsValue as String 
    // on failiure try to parse JsValue as Error
    v.as_string()
     .or_else(|| js_sys::Reflect::get(v, &JsValue::from_str("description"))
                         .ok().and_then(|x| x.as_string()))
     .unwrap_or_else(|| format!("error: {:?}", v))
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::WebSocketError(e) => write!(
                f, "{}", jsvalue_to_string(e)
            ),
        }
    }
}

impl From<JsValue> for ClientError {
    fn from(error: JsValue) -> Self {
        ClientError::WebSocketError(error)
    }
}
