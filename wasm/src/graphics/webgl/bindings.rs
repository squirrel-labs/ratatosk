use std::convert::TryInto;

use super::shader::ShaderType;
use crate::ClientError;
use rask_engine::math::Mat3;
use rask_engine::resources::{Texture, TextureRange};

pub struct Gl2;

impl Gl2 {
    pub const CONTEXT_LOST_WEBGL: u32 = 37442;
    pub const INVALID_ENUM: u32 = 1280;
    pub const INVALID_VALUE: u32 = 1281;
    pub const INVALID_OPERATION: u32 = 1282;
    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286;
    pub const NO_ERROR: u32 = 0;
    pub const OUT_OF_MEMORY: u32 = 1285;
    pub const FRAGMENT_SHADER: u32 = 35632;
    pub const VERTEX_SHADER: u32 = 35633;

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

    pub fn allocate_buffers(
        &self,
        matrices: &[Mat3],
        texture_bounds: &[(f32, f32, f32, f32)],
        texture_layers: &[u32],
    ) -> Result<(), ClientError> {
        let n = texture_layers.len();
        if matrices.len() != n * 9 || texture_bounds.len() != n * 4 {
            return Err(ClientError::WebGlError(
                "tried allocating incompatible buffer sizes".to_string(),
            ));
        }
        match unsafe {
            gl_allocate_buffers(
                matrices.as_ptr() as *const f32,
                texture_bounds.as_ptr() as *const f32,
                texture_layers.as_ptr(),
                n as u32,
            )
        } {
            0 => Ok(()),
            1 | 2 => Err(ClientError::WebGlError(
                "glCreateBuffer returned an unexpected object".to_string(),
            )),
            _ => unreachable!("unexpected return value from js function"),
        }
    }

    pub fn update_matrix_buffer(&self, matrices: &[Mat3]) {
        unsafe { gl_update_mat_buffer(matrices.as_ptr() as *const f32, matrices.len() as u32) }
    }

    pub fn update_texture_buffer(
        &self,
        texture_ranges: &[(f32, f32, f32, f32)],
        texture_layers: &[u32],
    ) {
        unsafe {
            gl_update_tex_buffer(
                texture_ranges.as_ptr() as *const f32,
                texture_layers.as_ptr() as *const u32,
                texture_ranges.len() as u32,
            )
        }
    }

    pub fn create_program(&self) -> Result<u32, ClientError> {
        unsafe { gl_create_program() }.try_into().map_err(|_| {
            ClientError::WebGlError("glCreateProgram returned an unexpected object".to_string())
        })
    }

    pub fn get_max_texture_size(&self) -> (u32, u32) {
        let w = unsafe { gl_query_max_texture_size() };
        (w, w)
    }

    pub fn attach_new_shader(&self, prog: u32, shader_type: ShaderType) -> Result<(), ClientError> {
        match unsafe { gl_attach_new_shader(prog as i32, shader_type as u32) } {
            0 => Ok(()),
            1 => Err(ClientError::WebGlError(
                "shader creation failed, invalid program handle".to_string(),
            )),
            2 => Err(ClientError::WebGlError(
                "shader creation failed, invalid/unknown shader type".to_string(),
            )),
            3 => Err(ClientError::WebGlError(
                "shader creation failed, glCreateShader returned an unexpected object".to_string(),
            )),
            4 => Err(ClientError::WebGlError(
                "shader compilation failed".to_string(),
            )),
            _ => unreachable!("unexpected return value from js function"),
        }
    }

    pub fn link_program(&self, prog: u32) -> Result<(), ClientError> {
        match unsafe { gl_link_program(prog as i32) } {
            0 => Ok(()),
            1 => Err(ClientError::WebGlError(
                "program linkage failed, invalid program handle".to_string(),
            )),
            2 => Err(ClientError::WebGlError(
                "program linkage failed, linker failed".to_string(),
            )),
            3 => Err(ClientError::WebGlError(
                "program linkage failed, shader attribute location not found".to_string(),
            )),
            _ => unreachable!("unexpected return value from js function"),
        }
    }

    pub fn realloc_texture_atlas(
        &self,
        w: u32,
        h: u32,
        layer_count: u32,
    ) -> Result<(), ClientError> {
        match unsafe { gl_realloc_texture_atlas(w, h, layer_count) } {
            0 => Ok(()),
            1 => Err(ClientError::WebGlError(
                "reallocation of texture buffer failed".to_string(),
            )),
            _ => unreachable!("unexpected return value from js function"),
        }
    }

    pub fn upload_texture_to_atlas(&self, range: TextureRange, layer: u32, texture: &Texture) {
        unsafe {
            gl_upload_texture_to_atlas(
                range.start.0,
                range.start.1,
                range.size.0,
                range.size.1,
                layer,
                texture.raw().as_ptr(),
            )
        };
    }
}

extern "C" {
    /// Same as `WebGLRenderingContext.getError()`.
    fn gl_get_error() -> u32;

    /// This function creates a vertex array with a vertex buffer.
    /// Return values are:
    ///     * 0 - success
    ///     * 1 - failure, VAO creation failed
    ///     * 2 - failure, VBO creation failed
    ///
    /// # Safety
    ///
    /// The pointer must be 32 bit aligned.
    fn gl_create_vertex_array_and_buffer_with_data(data: *const f32, len32: usize) -> u32;

    /// This function (re)allocates the matrix and texture buffers and initializes them with the
    /// given memory. If the buffers do not exist, they are created.
    /// Return values are:
    ///     * 0 - success
    ///     * 1 - failure, matrix buffer creation failed
    ///     * 2 - failure, texture buffer creation failed
    ///
    /// # Safety
    ///
    /// All pointers must be 32 bit aligned.
    fn gl_allocate_buffers(
        mat_ptr: *const f32,
        tex_bound_ptr: *const f32,
        tex_layer_ptr: *const u32,
        instances: u32,
    ) -> u32;

    /// This function updates the matrix buffer with the given values.
    ///
    /// # Safety
    ///
    /// All pointers must be 32 bit aligned.
    fn gl_update_mat_buffer(mat_ptr: *const f32, instances: u32);

    /// This function updates the texture buffer with the given values.
    ///
    /// # Safety
    ///
    /// All pointers must be 32 bit aligned.
    fn gl_update_tex_buffer(tex_bound_ptr: *const f32, tex_layer_ptr: *const u32, instances: u32);

    /// This function creates a shader program.
    /// Return values are:
    ///     * positive: success, a program handle was returned
    ///     * negative: failure, an error occurred
    fn gl_create_program() -> i32;

    /// This function compiles a shader and attaches it to a shader program.
    /// Return values are:
    ///     * 0 - success
    ///     * 1 - failure, invalid/unknown program handle
    ///     * 2 - failure, invalid/unknown shader type
    ///     * 3 - failure, shader creation failed
    ///     * 4 - failure, shader compilation failed
    fn gl_attach_new_shader(prog: i32, shader_type: u32) -> u32;

    /// This function links a shader program.
    /// Return values are:
    ///     * 0 - success
    ///     * 1 - failure, invalid/unknown program handle
    ///     * 2 - failure, program linkage failed
    ///     * 3 - failure, attribute location not found
    fn gl_link_program(prog: i32) -> u32;

    /// This function queries the maximum texture size supported by the gpu.
    /// Returns the size as width and height
    fn gl_query_max_texture_size() -> u32;

    /// This function (re)allocates the texture storage.
    /// Return values are:
    ///     * 0 - success
    ///     * 1 - failure, texture generation failed
    fn gl_realloc_texture_atlas(w: u32, h: u32, layer_count: u32) -> u32;

    /// This function draws the buffer onto the specified texture layer
    fn gl_upload_texture_to_atlas(
        start_x: u32,
        start_y: u32,
        width: u32,
        height: u32,
        layer: u32,
        buffer: *const u8,
    );
}
