use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum ClientError {
    WebSocketError(JsValue),
}

fn jsvalue_to_string(v: &JsValue) -> Option<String> {
    js_sys::Reflect::get(v, JsValue::from_str("description")).ok()?
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt:Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::WebSocketError(e) =>
                write!(f, "{}", jsvalue_to_string(e).unwrap_or("Unknown websocket error")),
        }
    }
}
