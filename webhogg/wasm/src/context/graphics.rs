use crate::error::WasmError;

pub struct GraphicsContext {
}

impl GraphicsContext {
    pub fn from_canvas() -> Result<Self, WasmError> {
        Ok(Self {
        })
    }
}
