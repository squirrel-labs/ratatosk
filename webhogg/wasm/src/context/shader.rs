use crate::error::WasmError;
use super::webgl;
use super::webgl::{WebGl2, ShaderType};

pub const MAIN_VERTEX_SHADER: &str = include_str!("main.vs");
pub const MAIN_FRAGMENT_SHADER: &str = include_str!("main.fs");

pub struct ShaderProgram {
    program: webgl::WebGlProgram,
}

impl ShaderProgram {
    pub fn from_sources(gl: &WebGl2, sources: &[(ShaderType, String)]) -> Result<Self, WasmError> {
        let program = gl.create_program()
            .map_err(|_| WasmError::Shader(format!("glCreateProgram failed ({})", gl.get_error())))?;
        for (shader_type, source) in sources {
            let shader = gl.create_shader(shader_type)
                .map_err(|_| WasmError::Shader(format!("glCreateShader failed ({})", gl.get_error())))?;
            gl.shader_source(&shader, source);
            gl.compile_shader(&shader)
                .map_err(|e| WasmError::Shader(format!("compile error in {} shader: {}", shader_type, e)))?;
            gl.attach_shader(&program, &shader)
        }
        gl.link_program(&program)
            .map_err(|e| WasmError::Shader(format!("linker error in program: {}", e)))?;
        Ok(Self {
            program
        })
    }

    pub fn run(&self, gl: &WebGl2) {
        gl.use_program(&self.program)
    }
}
