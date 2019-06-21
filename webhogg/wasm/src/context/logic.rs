use crate::error::WasmError;

pub struct LogicContext {
}

impl LogicContext {
    pub fn new() -> Result<Self, WasmError> {
        Ok(Self {
        })
    }
}
