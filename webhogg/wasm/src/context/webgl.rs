pub use web_sys::{
    WebGl2RenderingContext as GlContext,
    WebGlProgram, WebGlShader,
    WebGlBuffer, WebGlVertexArrayObject,
    WebGlUniformLocation
};
use wasm_bindgen::prelude::*;
use std::fmt::Display;

pub struct Color4(f32, f32, f32, f32);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=Float32Array, js_name=of, variadic)]
    fn _create_f32_buffer(args: &[f32]) -> js_sys::Float32Array;
}

#[derive(Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl Display for ShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ShaderType {
    pub fn to_id(&self) -> u32 {
        match self {
            ShaderType::Vertex => GlContext::VERTEX_SHADER,
            ShaderType::Fragment => GlContext::FRAGMENT_SHADER,
        }
    }
}

impl Color4 {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color4 {
        Color4(r, g, b, a)
    }
}

pub fn create_f32_buffer(buffer: &[f32]) -> js_sys::Float32Array {
    _create_f32_buffer(buffer)
}

pub struct WebGlError {
    num: u32,
}

impl Display for WebGlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self.num {
            GlContext::NO_ERROR => "NO_ERROR",
            GlContext::INVALID_OPERATION => "INVALID_OPERATION",
            GlContext::INVALID_ENUM => "INVALID_ENUM",
            GlContext::INVALID_VALUE => "INVALID_VALUE",
            GlContext::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            GlContext::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            _ => "UNKNOWN ERROR"
        })
    }
}

impl From<u32> for WebGlError {
    fn from(n: u32) -> Self {
        WebGlError { num: n }
    }
}

impl WebGlError {
    pub fn is_ok(&self) -> bool { self.num == 0 }
    pub fn is_err(&self) -> bool { self.num != 0 }
}

pub struct WebGl2 {
    pub gl: GlContext,
}

impl WebGl2 {
    pub fn from_context(context: GlContext) -> Self {
        WebGl2 {
            gl: context,
        }
    }

    pub fn create_program(&self) -> Result<WebGlProgram, ()> {
        self.gl.create_program().ok_or(())
    }

    pub fn create_shader(&self, shader_type: &ShaderType) -> Result<WebGlShader, ()> {
        self.gl.create_shader(shader_type.to_id()).ok_or(())
    }

    pub fn get_error(&self) -> WebGlError { self.gl.get_error().into() }
    pub fn shader_source(&self, id: &WebGlShader, source: &str) { self.gl.shader_source(id, source) }
    pub fn compile_shader(&self, id: &WebGlShader) -> Result<(), String> {
        self.gl.compile_shader(id);
        if self.gl.get_shader_parameter(id, GlContext::COMPILE_STATUS) == JsValue::FALSE {
            Err(self.gl.get_shader_info_log(id)
                .unwrap_or("/could not retrieve program information/".to_string()))
        } else { Ok(()) }
    }
    pub fn link_program(&self, id: &WebGlProgram) -> Result<(), String> {
        self.gl.link_program(id);
        if self.gl.get_program_parameter(id, GlContext::LINK_STATUS) == JsValue::FALSE {
            Err(self.gl.get_program_info_log(id)
                .unwrap_or("/could not retrieve program information/".to_string()))
        } else { Ok(()) }
    }
    pub fn attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
        self.gl.attach_shader(program, shader)
    }

    pub fn clear(&self, color: &Color4) {
        self.gl.clear_color(color.0, color.1, color.2, color.3);
        self.gl.clear(GlContext::COLOR_BUFFER_BIT);
    }

    pub fn set_viewport(&self) {
        self.gl.viewport(0, 0, self.gl.drawing_buffer_width(), self.gl.drawing_buffer_height());
    }

    pub fn create_buffer(&self) -> Result<WebGlBuffer, ()> {
        self.gl.create_buffer().ok_or(())
    }

    pub fn bind_array_buffer(&self, buffer: &WebGlBuffer) {
        self.gl.bind_buffer(GlContext::ARRAY_BUFFER, Some(buffer))
    }
    pub fn unbind_array_buffer(&self) { self.gl.bind_buffer(GlContext::ARRAY_BUFFER, None) }

    pub fn array_buffer_data_f32(&self, data: &[f32]) {
        self.gl.buffer_data_with_opt_array_buffer(
            GlContext::ARRAY_BUFFER,
            Some(&create_f32_buffer(data).buffer()),
            GlContext::STATIC_DRAW)
    }

    pub fn create_vertex_array(&self) -> Result<WebGlVertexArrayObject, ()> {
        self.gl.create_vertex_array().ok_or(())
    }
    pub fn bind_vertex_array(&self, array: &WebGlVertexArrayObject) {
        self.gl.bind_vertex_array(Some(array))
    }
    pub fn unbind_vertex_array(&self) { self.gl.bind_vertex_array(None) }
    pub fn vertex_attrib_f32_pointer(&self, location: u32, dim: i32) {
        self.gl.vertex_attrib_pointer_with_i32(location, dim, GlContext::FLOAT, false, 0, 0)
    }

    pub fn draw_triangle_arrays(&self, count: i32) {
        self.gl.draw_arrays(GlContext::TRIANGLES, 0, count)
    }

    pub fn enable_vertex_attrib_array(&self, location: u32) {
        self.gl.enable_vertex_attrib_array(location)
    }

    pub fn use_program(&self, program: &WebGlProgram) {
        self.gl.use_program(Some(program))
    }

    pub fn uniform_f32v2(&self, location: &WebGlUniformLocation, data: &[f32]) {
        self.gl.uniform2fv_with_f32_array(Some(location), data)
    }
}
