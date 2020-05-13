use rask_engine::math::Mat3;
use crate::error::ClientError;
use web_sys::WebGl2RenderingContext as Gl2;
use web_sys::WebGlShader;
use web_sys::{WebGlProgram, WebGlUniformLocation};

const VERTEX_SHADER: &str = include_str!("shader/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("shader/fragment.glsl");

enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Debug)]
pub struct Program {
    id: WebGlProgram,
    transformation: WebGlUniformLocation,
    texture: WebGlUniformLocation,
}

impl Program {
    fn get_uniform_location(
        gl: &Gl2,
        prog: &WebGlProgram,
        name: &str,
    ) -> Result<WebGlUniformLocation, ClientError> {
        gl.get_uniform_location(&prog, name)
            .ok_or(ClientError::WebGlError(format!(
                "cannot find uniform location \"{}\"",
                name
            )))
    }

    pub fn new(gl: &Gl2) -> Result<Self, ClientError> {
        let prog = gl.create_program().ok_or(ClientError::WebGlError(
            "cannot create a webgl shader program".to_owned(),
        ))?;
        let vs = Shader::create_from_source(gl, VERTEX_SHADER, ShaderType::Vertex)?;
        let fs = Shader::create_from_source(gl, FRAGMENT_SHADER, ShaderType::Fragment)?;
        vs.attach(&gl, &prog);
        fs.attach(&gl, &prog);
        gl.link_program(&prog);

        const TRANSFORMATION: &str = "transformation";
        const TEXTURE: &str = "g_texture";

        if gl.get_program_parameter(&prog, Gl2::LINK_STATUS).as_bool() == Some(true) {
            Ok(Self {
                transformation: Self::get_uniform_location(&gl, &prog, TRANSFORMATION)?,
                texture: Self::get_uniform_location(&gl, &prog, TEXTURE)?,
                id: prog,
            })
        } else {
            let info = gl
                .get_program_info_log(&prog)
                .unwrap_or("<undefined>".to_owned());
            Err(ClientError::WebGlError(format!("link error: {}", info)))
        }
    }

    pub fn use_program(&self, gl: &Gl2) {
        gl.use_program(Some(&self.id))
    }

    pub fn upload_fransformation(&self, gl: &Gl2, mat: &Mat3) {
        self.use_program(gl);
        gl.uniform_matrix3fv_with_f32_array(Some(&self.transformation), true, &mat.as_ref().clone())
    }

    pub fn upload_texture_id(&self, gl: &Gl2, id: i32) {
        self.use_program(gl);
        gl.uniform1i(Some(&self.texture), id)
    }
}

struct Shader(WebGlShader);

impl Shader {
    pub fn create_from_source(
        gl: &Gl2,
        source: &str,
        shader_type: ShaderType,
    ) -> Result<Self, ClientError> {
        let type_ = match shader_type {
            ShaderType::Vertex => Gl2::VERTEX_SHADER,
            ShaderType::Fragment => Gl2::FRAGMENT_SHADER,
        };
        let shader = gl.create_shader(type_).ok_or(ClientError::WebGlError(
            "cannot create a webgl shader".to_owned(),
        ))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        if gl
            .get_shader_parameter(&shader, Gl2::COMPILE_STATUS)
            .as_bool()
            == Some(true)
        {
            Ok(Self(shader))
        } else {
            let info = gl
                .get_shader_info_log(&shader)
                .unwrap_or("<undefined>".to_owned());
            Err(ClientError::WebGlError(format!("comile error: {}", info)))
        }
    }

    pub fn attach(&self, gl: &Gl2, prog: &WebGlProgram) {
        gl.attach_shader(prog, &self.0)
    }
}