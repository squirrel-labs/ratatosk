use crate::shader::Program;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as Gl2;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject as Vao;
use webhogg_wasm_shared::error::ClientError;

pub trait GraphicsApi: Sized {
    fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError>;

    fn clear(&self) -> Result<(), ClientError>;
}

pub struct WebGl {
    gl: Gl2,
    vao: Vao,
    vbo: WebGlBuffer,
    prog: Program,
    canvas: web_sys::OffscreenCanvas,
}

impl GraphicsApi for WebGl {
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

    fn clear(&self) -> Result<(), ClientError> {
        self.gl.clear(Gl2::COLOR_BUFFER_BIT);
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

        Ok((vao, vbo))
    }

    fn create_program(gl: &Gl2) -> Result<Program, ClientError> {
        Program::new(gl)
    }

    fn buffer_data_with_f32_array(gl: &Gl2, arr: &[f32]) -> Result<(), ClientError> {
        gl.buffer_data_with_u8_array(
            Gl2::ARRAY_BUFFER,
            unsafe { std::slice::from_raw_parts(arr.as_ptr() as *const u8, 4 * arr.len()) },
            Gl2::STATIC_DRAW,
        );
        Ok(())
    }
}
