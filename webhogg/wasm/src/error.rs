use std::error::Error;

#[derive(Debug)]
pub enum WasmError {
    WebGl2ContextCreation(String),
}

impl std::fmt::Display for WasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}: {}", self.name(), self.description())
    }
}

impl Error for WasmError {
    fn description(&self) -> &str {
        match self {
            WasmError::WebGl2ContextCreation(msg) => msg,
        }
    }

    fn source(&self) -> Option<&'static dyn Error> { None }
}

impl WasmError {
    pub fn name(&self) -> &str {
        match self {
            WasmError::WebGl2ContextCreation(_) => "WebGl2ContextCreationError",
        }
    }
}
