use super::Gl2;
use crate::ClientError;

#[repr(u32)]
pub enum ShaderType {
    Vertex = Gl2::VERTEX_SHADER,
    Fragment = Gl2::FRAGMENT_SHADER,
}

/// A WebGL Shader program is a program running on an GPU device.
/// It is linked from multiple shaders.
pub struct ShaderProgram {
    handle: u32,
    transform_loc_index: u32,
    texture_loc_index: u32,
}

impl ShaderProgram {
    pub fn new(gl: &Gl2) -> Result<Self, ClientError> {
        let prog = gl.create_program()?;
        gl.attach_new_shader(prog, ShaderType::Vertex)?;
        gl.attach_new_shader(prog, ShaderType::Fragment)?;
        gl.link_program(prog)?;
        Ok(Self {
            handle: prog,
            transform_loc_index: 0, // TODO
            texture_loc_index: 0,   // TODO
        })
    }
}
