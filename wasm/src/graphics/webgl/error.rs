use super::bindings::Gl2;

#[derive(Debug)]
pub enum WebGl2Error {
    ContextLost,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    UnknownError,
}

impl From<u32> for WebGl2Error {
    fn from(v: u32) -> Self {
        match v {
            Gl2::INVALID_ENUM => WebGl2Error::InvalidEnum,
            Gl2::INVALID_VALUE => WebGl2Error::InvalidValue,
            Gl2::INVALID_OPERATION => WebGl2Error::InvalidOperation,
            Gl2::INVALID_FRAMEBUFFER_OPERATION => WebGl2Error::InvalidFramebufferOperation,
            Gl2::OUT_OF_MEMORY => WebGl2Error::OutOfMemory,
            Gl2::CONTEXT_LOST_WEBGL => WebGl2Error::ContextLost,
            _ => WebGl2Error::UnknownError,
        }
    }
}

impl std::fmt::Display for WebGl2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WebGl2Error::ContextLost => "lost webgl context",
                WebGl2Error::InvalidEnum => "invalid enum",
                WebGl2Error::InvalidValue => "invalid value",
                WebGl2Error::InvalidOperation => "invalid operation",
                WebGl2Error::InvalidFramebufferOperation => "invalid framebuffer operation",
                WebGl2Error::OutOfMemory => "out of memory",
                WebGl2Error::UnknownError => "unknown webgl2 error",
            }
        )
    }
}
