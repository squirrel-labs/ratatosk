use crate::error::WasmError;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GlContext;

use super::draw_sprite::DrawSprite;
use super::matrix::Matrix;
use super::shader::ShaderProgram;
use super::shader::{MAIN_FRAGMENT_SHADER, MAIN_VERTEX_SHADER};
use super::webgl;
use super::webgl::{Color4, ShaderType, WebGl2};

pub struct GraphicsContext {
    gl: WebGl2,
    frame_nr: u64,
    shader: ShaderProgram,
    vao: webgl::WebGlVertexArrayObject,
    buffer: webgl::WebGlBuffer,
    sprites: Vec<DrawSprite>,
}

impl GraphicsContext {
    pub fn from_canvas(canvas: web_sys::OffscreenCanvas) -> Result<Self, WasmError> {
        let context = canvas
            .get_context("webgl2")
            .map_err(|_| {
                WasmError::WebGl2ContextCreation(format!(
                    "context cration failed: getContext returned an exception"
                ))
            })?
            .ok_or_else(|| {
                WasmError::WebGl2ContextCreation(format!(
                    "context cration failed: getContext returned nothing"
                ))
            })?;
        let context = context.dyn_into::<GlContext>().map_err(|_| {
            WasmError::WebGl2ContextCreation(format!("context object is not a context"))
        })?;

        let gl = WebGl2::from_context(context);
        let shader = ShaderProgram::from_sources(
            &gl,
            &[
                (ShaderType::Vertex, MAIN_VERTEX_SHADER.to_string()),
                (ShaderType::Fragment, MAIN_FRAGMENT_SHADER.to_string()),
            ],
        )?;

        let vao = gl
            .create_vertex_array()
            .map_err(|_| WasmError::WebGlBuffer(format!("glGenVertexArrays failed")))?;
        gl.bind_vertex_array(&vao);

        let buffer = gl
            .create_buffer()
            .map_err(|_| WasmError::WebGlBuffer(format!("glCreateBuffer failed")))?;
        gl.bind_array_buffer(&buffer);
        gl.array_buffer_data_f32(&[0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0]);
        gl.enable_vertex_attrib_array(0);
        //gl.unbind_array_buffer();
        //gl.unbind_vertex_array();

        let sprites = vec![
            DrawSprite::new((0.0, 0.0), Matrix::identity()),
            //DrawSprite::new((0.5, 0.0), Matrix::identity()),
        ];

        Ok(Self {
            gl,
            frame_nr: 0,
            shader,
            vao,
            buffer,
            sprites,
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
        self.gl.clear(&Color4::new(b, light - a, b, 1.0));

        self.shader.run(&self.gl);
        self.gl.bind_vertex_array(&self.vao);
        self.gl.bind_array_buffer(&self.buffer);
        self.gl.enable_vertex_attrib_array(0);

        let bp = (
            f64::sin(self.frame_nr as f64 * 0.01) as f32,
            f64::cos(self.frame_nr as f64 * 0.01) as f32,
        );
        self.sprites[0].transform = Matrix::identity().translate(bp);

        for sprite in self.sprites.iter() {
            let pos = sprite.pos;
            let transform = &sprite.transform;

            let loc =
                self.shader
                    .get_location(&self.gl, "offset")
                    .ok_or(WasmError::WebGlUniform(format!(
                        "could not find location \"offset\" in glsl shader"
                    )))?;
            self.gl.uniform_f32v2(&loc, &[pos.0, pos.1]);

            let loc =
                self.shader
                    .get_location(&self.gl, "transform")
                    .ok_or(WasmError::WebGlUniform(format!(
                        "could not find location \"transform\" in glsl shader"
                    )))?;
            self.gl.uniform_mat3x3(&loc, &transform.raw);

            self.gl.vertex_attrib_f32_pointer(0, 2);
            self.gl.draw_triangle_arrays(6);
        }

        use log::info;
        let err = self.gl.get_error();
        if err.is_err() {
            info!("gl error: {}", err);
        }

        self.frame_nr += 1;

        Ok(())
    }
}
