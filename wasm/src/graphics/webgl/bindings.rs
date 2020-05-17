use crate::ClientError;

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

    pub fn create_vao_with_buffer_data(&self, data: &[f32]) -> Result<(), ClientError> {
        match unsafe { gl_create_vertex_array_and_buffer_with_data(data.as_ptr(), data.len()) } {
            0 => Ok(()),
            1 => Err(ClientError::WebGlError(
                "glCreateVertexArray returned an unexpected object".to_string(),
            )),
            2 => Err(ClientError::WebGlError(
                "glCreateBuffer returned an unexpected object".to_string(),
            )),
            _ => unreachable!("unexpected return value from js function"),
        }
    }
}

extern "C" {
    /// same as `WebGLRenderingContext.getError()`
    fn gl_get_error() -> u32;
    /// This function creates a vertex array with a vertex buffer.
    /// It returns an error code:
    ///     * 0 - success
    ///     * 1 - failiure, VAO creation failed
    ///     * 2 - failiure, VBO creation failed
    /// # Safety
    /// The pointer must be 32bit aligned
    fn gl_create_vertex_array_and_buffer_with_data(data: *const f32, len32: usize) -> u32;
}
