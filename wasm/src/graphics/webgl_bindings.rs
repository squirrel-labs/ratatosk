pub struct Gl2;

impl Gl2 {
    pub const CONTEXT_LOST_WEBGL: u32 = 37442;
    pub const INVALID_ENUM: u32 = 1280;
    pub const INVALID_VALUE: u32 = 1281;
    pub const INVALID_OPERATION: u32 = 1282;
    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286;
    pub const NO_ERROR: u32 = 0;
    pub const OUT_OF_MEMORY: u32 = 1285;

    pub fn get_error(&self) -> u32 {
        unsafe { gl_get_error() }
    }
}

extern "C" {
    fn gl_get_error() -> u32;
}
