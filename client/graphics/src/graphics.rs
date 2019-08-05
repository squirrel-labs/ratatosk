use crate::shader::Program;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as Gl2;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject as Vao;
use webhogg_wasm_shared::error::ClientError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=Float32Array, js_name=of, variadic)]
    fn _create_f32_buffer(args: &[f32]) -> js_sys::Float32Array;
}

pub enum WebGl2Error {
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

pub trait GraphicsApi: Sized {
    type GraphicsError: std::fmt::Display;

    fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError>;

    fn clear(&self) -> Result<(), ClientError>;
    fn draw_rect(&self) -> Result<(), ClientError>;
    fn ok(&self) -> Result<(), Self::GraphicsError>;
}

pub struct WebGl {
    gl: Gl2,
    vao: Vao,
    vbo: WebGlBuffer,
    prog: Program,
    canvas: web_sys::OffscreenCanvas,
}

impl GraphicsApi for WebGl {
    type GraphicsError = WebGl2Error;

    fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError> {
        let gl: Gl2 = canvas
            .get_context("webgl2")?
            .ok_or(ClientError::WebGlError(
                "getContext returns nothing, webgl2 doesn't seem to be supported".to_owned(),
            ))?
            .dyn_into()
            .ok()
            .ok_or(ClientError::WebGlError(
                "getContext returns invalid data type, webgl2 doesn't seem to be supported"
                    .to_owned(),
            ))?;
        gl.clear_color(0.8, 0.2, 0.7, 1.0);

        let (vao, vbo) = Self::create_vao(&gl)?;
        let prog = Self::create_program(&gl)?;

        Ok(WebGl {
            canvas,
            gl,
            vao,
            prog,
            vbo,
        })
    }

    fn ok(&self) -> Result<(), Self::GraphicsError> {
        match self.gl.get_error() {
            Gl2::NO_ERROR => Ok(()),
            e => Err(e.into()),
        }
    }

    fn clear(&self) -> Result<(), ClientError> {
        self.gl.clear(Gl2::COLOR_BUFFER_BIT);
        Ok(())
    }

    fn draw_rect(&self) -> Result<(), ClientError> {
        self.prog.use_program(&self.gl);
        self.gl.bind_vertex_array(Some(&self.vao));
        self.gl.bind_buffer(Gl2::ARRAY_BUFFER, Some(&self.vbo));
        self.gl
            .vertex_attrib_pointer_with_i32(0, 2, Gl2::FLOAT, false, 0, 0);
        self.gl.draw_arrays(Gl2::TRIANGLES, 0, 6);
        Ok(())
    }
}

impl WebGl {
    fn create_vao(gl: &Gl2) -> Result<(Vao, WebGlBuffer), ClientError> {
        let vao = gl.create_vertex_array().ok_or(ClientError::WebGlError(
            "cannot create a webgl vertex array object".to_owned(),
        ))?;
        gl.bind_vertex_array(Some(&vao));
        let vbo = gl.create_buffer().ok_or(ClientError::WebGlError(
            "cannot create a webgl vertex buffer".to_owned(),
        ))?;
        gl.bind_buffer(Gl2::ARRAY_BUFFER, Some(&vbo));
        Self::buffer_data_with_f32_array(
            &gl,
            &[
                -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
            ],
        )?;
        gl.enable_vertex_attrib_array(0);

        Ok((vao, vbo))
    }

    fn create_program(gl: &Gl2) -> Result<Program, ClientError> {
        Program::new(gl)
    }

    fn buffer_data_with_f32_array(gl: &Gl2, arr: &[f32]) -> Result<(), ClientError> {
        gl.buffer_data_with_opt_array_buffer(
            Gl2::ARRAY_BUFFER,
            Some(&_create_f32_buffer(arr).buffer()),
            Gl2::STATIC_DRAW,
        );
        Ok(())
    }
}
