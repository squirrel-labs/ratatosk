use super::bindings::Gl2;

macro_rules! error_variants {
    ($iv:vis $st:ident { $($v:ident($c:expr) -> $s:literal),+ }) => {
        #[repr(u32)]
        #[derive(Debug)]
        $iv enum $st {
            $($v = $c),+
        }
        impl From<u32> for $st {
            fn from(v: u32) -> Self {
                match v {
                    $(x if x == $st::$v as u32 => $st::$v),+,
                    _ => $st::UnknownError,
                }
            }
        }
        impl std::fmt::Display for $st {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $($st::$v => $s),+
                })
            }
        }
    };
}

error_variants!(pub WebGl2Error {
    ContextLost(Gl2::CONTEXT_LOST_WEBGL) -> "lost webgl context",
    InvalidEnum(Gl2::INVALID_ENUM) -> "invalid enum",
    InvalidValue(Gl2::INVALID_VALUE) -> "invalid value",
    InvalidOperation(Gl2::INVALID_OPERATION) -> "invalid operation",
    InvalidFramebufferOperation(Gl2::INVALID_FRAMEBUFFER_OPERATION) -> "invalid framebuffer operation",
    OutOfMemory(Gl2::OUT_OF_MEMORY) -> "out of memory",
    UnknownError(0xff) -> "unknown webgl2 error"
});
