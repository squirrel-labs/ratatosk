use crate::error::WasmError;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GlContext;

use super::webgl;
use super::webgl::{Color4, ShaderType, WebGl2};
use super::shader::{MAIN_VERTEX_SHADER, MAIN_FRAGMENT_SHADER};
use super::shader::ShaderProgram;

pub struct GraphicsContext {
    gl: WebGl2,
    frame_nr: u64,
    shader: ShaderProgram,
    vao: webgl::WebGlVertexArrayObject,
    buffer: webgl::WebGlBuffer,
}

impl GraphicsContext {
    pub fn from_canvas(canvas: web_sys::OffscreenCanvas) -> Result<Self, WasmError> {
        let context = canvas.get_context("webgl2")
            .map_err(|_| WasmError::WebGl2ContextCreation(
                    format!("context cration failed: getContext returned an exception")))?
            .ok_or_else(|| WasmError::WebGl2ContextCreation(
                    format!("context cration failed: getContext returned nothing")))?;
        let context = context
            .dyn_into::<GlContext>()
            .map_err(|_| WasmError::WebGl2ContextCreation(
                    format!("context object is not a context")))?;

        let gl = WebGl2::from_context(context);
        let shader = ShaderProgram::from_sources(&gl, &[
            (ShaderType::Vertex, MAIN_VERTEX_SHADER.to_string()),
            (ShaderType::Fragment, MAIN_FRAGMENT_SHADER.to_string()),
        ])?;

        let vao = gl.create_vertex_array()
            .map_err(|_| WasmError::WebGlBuffer(
                    format!("glGenVertexArrays failed")))?;
        gl.bind_vertex_array(&vao);

        let buffer = gl.create_buffer()
            .map_err(|_| WasmError::WebGlBuffer(
                    format!("glCreateBuffer failed")))?;
        gl.bind_array_buffer(&buffer);
        gl.array_buffer_data_f32(&[
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            1.0, 0.0,
        ]);
        gl.enable_vertex_attrib_array(0);
            
        Ok(Self {
            gl, frame_nr: 0,
            shader, vao, buffer
        })
    }

    pub fn update(&mut self) -> Result<(), WasmError> {
        let light = 0.5;
        let speed = 30.0;

        let a = (self.frame_nr as f32) / speed;
        let a = f32::abs(f32::sin(a));
        let b = f32::abs(f32::cos(a));
        let (a, b) = (a * light, b * light);

        self.gl.set_viewport();
        self.gl.clear(&Color4::new(a, light - a, b, 1.0));

        self.shader.run(&self.gl);
        self.gl.vertex_attrib_f32_pointer(0, 2);
        self.gl.draw_triangle_arrays(6);

        self.frame_nr += 1;

        Ok(())
    }
}
