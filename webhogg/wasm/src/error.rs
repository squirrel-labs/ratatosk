use std::error::Error;

#[derive(Debug)]
pub enum WasmError {
    WebGl2ContextCreation(String),
    Shader(String),
    WebGlBuffer(String),
    WebGlUniform(String),
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
            WasmError::Shader(msg) => msg,
            WasmError::WebGlBuffer(msg) => msg,
            WasmError::WebGlUniform(msg) => msg,
        }
    }

    fn source(&self) -> Option<&'static dyn Error> {
        None
    }
}

impl WasmError {
    pub fn name(&self) -> &str {
        match self {
            WasmError::WebGl2ContextCreation(_) => "WebGl2ContextCreationError",
            WasmError::Shader(_) => "ShaderError",
            WasmError::WebGlBuffer(_) => "WebGlBufferError",
            WasmError::WebGlUniform(_) => "WebGlUniformError",
        }
    }
}
