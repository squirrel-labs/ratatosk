use crate::shader::Program;
use rask_engine::math;
use rask_engine::math::Mat3;
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::state::State;
use rask_wasm_shared::texture::{Texture, ColorType};
use rask_wasm_shared::sprite::TextureId;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as Gl2;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject as Vao;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=Float32Array, js_name=of, variadic)]
    fn _create_f32_buffer(args: &[f32]) -> js_sys::Float32Array;
    #[wasm_bindgen(js_namespace=Uint8Array, js_name=of, variadic)]
    fn _create_u8_buffer(args: &[u8]) -> js_sys::Uint8Array;
}

#[derive(Debug)]
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

    fn new(canvas: web_sys::OffscreenCanvas, size_multiplicator: math::vec2::Vec2) -> Result<Self, ClientError>;

    fn start_frame(&mut self, color: &[f32; 3]) -> Result<(), ClientError>;
    fn end_frame(&self) -> Result<(), ClientError>;
    fn draw_rect(&self, pos: &math::Vec2, mat: &Mat3, tex: u32) -> Result<(), ClientError>;
    fn upload_texture(&mut self, texture: &mut Texture, n: u32) -> Result<(), ClientError>;
    fn resize_texture_pool(&mut self, n: u32) -> Result<(), ClientError>;
    fn ok(&self) -> Result<(), Self::GraphicsError>;
}

struct GlFramebuffer {
    fb: web_sys::WebGlFramebuffer,
    rb: web_sys::WebGlRenderbuffer,
    w: i32, h: i32,
    tex: WebGlApiTexture
}

impl GlFramebuffer {
    fn new(gl: &Gl2, w: u32, h: u32) -> Result<Self, ClientError> {
        let (w, h) = (w as i32, h as i32);
        let tex = WebGlApiTexture::new(gl)?;
        tex.bind(gl);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            Gl2::TEXTURE_2D, 0, Gl2::RGB8 as i32, w, h, 0, Gl2::RGB, Gl2::UNSIGNED_BYTE, None
            )?;

        gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_MIN_FILTER, Gl2::NEAREST as i32);
        gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_MAG_FILTER, Gl2::NEAREST as i32);
        gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_WRAP_S, Gl2::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_WRAP_T, Gl2::CLAMP_TO_EDGE as i32);

        let fb = gl.create_framebuffer()
            .ok_or(ClientError::WebGlError(format!("could not create a framebuffer object")))?;
        gl.bind_framebuffer(Gl2::FRAMEBUFFER, Some(&fb));
        tex.attach_framebuffer(gl, Gl2::COLOR_ATTACHMENT0);

        let rb = gl.create_renderbuffer()
            .ok_or(ClientError::WebGlError(format!("could not create a renderbuffer object")))?;
        gl.bind_renderbuffer(Gl2::RENDERBUFFER, Some(&rb));
        gl.renderbuffer_storage(Gl2::RENDERBUFFER, Gl2::DEPTH_COMPONENT16, w, h);
        gl.bind_renderbuffer(Gl2::RENDERBUFFER, None);
        gl.framebuffer_renderbuffer(Gl2::FRAMEBUFFER, Gl2::DEPTH_ATTACHMENT, Gl2::RENDERBUFFER, Some(&rb));

        Ok(Self {
            fb, rb,
            w, h,
            tex
        })
    }

    pub fn render_pass_0(&self, gl: &Gl2) {
        gl.bind_framebuffer(Gl2::FRAMEBUFFER, Some(&self.fb));
        gl.viewport(0, 0, self.w, self.h);
    }

    pub fn render_pass_1(&self, gl: &Gl2) {
        gl.bind_framebuffer(Gl2::FRAMEBUFFER, None);
        self.tex.bind(&gl);

        let framebuffer_status = gl.check_framebuffer_status(Gl2::FRAMEBUFFER);
        if framebuffer_status != Gl2::FRAMEBUFFER_COMPLETE {
            log::error!("framebuffer failure {}", framebuffer_status);
        }
    }
}

pub struct WebGl {
    gl: Gl2,
    fb: GlFramebuffer,
    vao: Vao,
    vbo: WebGlBuffer,
    prog: Program,
    width: u32, height: u32,
    canvas: web_sys::OffscreenCanvas,
    texture_handles: Vec<Option<WebGlApiTexture>>,
}

impl GraphicsApi for WebGl {
    type GraphicsError = WebGl2Error;

    fn new(canvas: web_sys::OffscreenCanvas, size_multiplicator: math::vec2::Vec2) -> Result<Self, ClientError> {
        let (width, height) = (canvas.width(), canvas.height());
        let (target_width, target_height) = (
            ((width as f32) * size_multiplicator.x()) as u32,
            ((height as f32) * size_multiplicator.y()) as u32);
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

        let (vao, vbo) = Self::create_vao(&gl)?;
        let prog = Self::create_program(&gl)?;

        let fb = GlFramebuffer::new(&gl, target_width, target_height)?;

        prog.use_program(&gl);
        gl.vertex_attrib_pointer_with_i32(0, 2, Gl2::FLOAT, false, 0, 0);

        Ok(WebGl {
            canvas,
            gl,
            fb,
            vao,
            prog,
            width, height,
            vbo,
            texture_handles: vec![],
        })
    }

    fn upload_texture(&mut self, texture: &mut Texture, n: u32) -> Result<(), ClientError> {
        let handle = WebGlApiTexture::new(&self.gl)?;
        self.gl.active_texture(Gl2::TEXTURE0);
        handle.bind(&self.gl);
        if let ColorType::RGB(_) = texture.colortype() {
            // TODO: copy RGB buffer to RGBA
            return Err(ClientError::ResourceError(format!("RGB not yet implemented")));
        }
        let (internalformat, format) = Self::colorformat(texture.colortype())?;
        let (w, h) = texture.dimension();
        self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            Gl2::TEXTURE_2D, 0, internalformat, w as i32, h as i32, 0, format, Gl2::UNSIGNED_BYTE, 
            Some(&texture.raw()))?;
        self.gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_WRAP_S, Gl2::CLAMP_TO_EDGE as i32);
        self.gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_WRAP_T, Gl2::CLAMP_TO_EDGE as i32);
        self.gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_MIN_FILTER, Gl2::NEAREST as i32);
        self.gl.tex_parameteri(Gl2::TEXTURE_2D, Gl2::TEXTURE_MAG_FILTER, Gl2::NEAREST as i32);
        self.texture_handles[n as usize] = Some(handle);
        Ok(())
    }

    fn resize_texture_pool(&mut self, n: u32) -> Result<(), ClientError> {
        let n = n as usize;
        if self.texture_handles.len() < n {
            self.texture_handles.resize(n, None)
        }
        Ok(())
    }

    fn ok(&self) -> Result<(), Self::GraphicsError> {
        Self::_ok(&self.gl)
    }

    fn start_frame(&mut self, color: &[f32; 3]) -> Result<(), ClientError> {
        self.fb.render_pass_0(&self.gl);

        self.gl.clear_color(color[0], color[1], color[2], 1.0);
        self.gl.clear(Gl2::COLOR_BUFFER_BIT);
        Ok(())
    }

    fn end_frame(&self) -> Result<(), ClientError> {
        self.fb.render_pass_1(&self.gl);
        self.gl.viewport(0, 0, self.width as i32, self.height as i32);
        self.draw_rect_notexture(&math::Vec2::new(0.0, 0.0), &-Mat3::identity())
    }

    fn draw_rect(&self, pos: &math::Vec2, mat: &Mat3, tex: TextureId) -> Result<(), ClientError> {
        self.bind_texture(tex);
        self.draw_rect_notexture(pos, mat)
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

    fn bind_texture(&self, tex: TextureId) -> Result<(), ClientError> {
        Ok(self.texture_handles
            .get(tex as usize)
            .ok_or_else(|| ClientError::ResourceError(format!("texture #{} is out of bounds", tex)))?
            .as_ref()
            .ok_or_else(|| ClientError::ResourceError(format!("could not get texture #{}", tex)))?
            .bind(&self.gl))
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

    pub(self) fn _ok(gl: &Gl2) -> Result<(), WebGl2Error> {
        match gl.get_error() {
            Gl2::NO_ERROR => Ok(()),
            e => Err(e.into()),
        }
    }

    fn colorformat(format: ColorType) -> Result<(i32, u32), ClientError> {
        match format {
            ColorType::RGB(8) => Ok((Gl2::RGB8 as i32, Gl2::RGB)),
            ColorType::RGB(16) => Ok((Gl2::RGB16UI as i32, Gl2::RGB)),

            ColorType::RGBA(8) => Ok((Gl2::RGBA8 as i32, Gl2::RGBA)),
            ColorType::RGBA(16) => Ok((Gl2::RGBA16UI as i32, Gl2::RGBA)),
            ColorType::RGBA(32) => Ok((Gl2::RGBA32UI as i32, Gl2::RGBA)),
            _ => Err(ClientError::WebGlError(format!("invalid color format")))
        }
    }

    fn draw_rect_notexture(&self, pos: &math::Vec2, mat: &Mat3) -> Result<(), ClientError> {
        self.prog.upload_fransformation(&self.gl, mat);
        self.prog.upload_texture_id(&self.gl, 0);
        self.gl.vertex_attrib2fv_with_f32_array(1, &[pos.x(), pos.y()]);
        self.gl.draw_arrays(Gl2::TRIANGLES, 0, 6);
        Ok(())
    }
}

#[derive(Clone)]
pub struct WebGlApiTexture(web_sys::WebGlTexture);

impl WebGlApiTexture {
    pub fn new(gl: &Gl2) -> Result<Self, ClientError> {
        Ok(Self(gl.create_texture()
            .ok_or(ClientError::WebGlError(format!("could not create a texture handle")))?))
    }

    pub fn bind(&self, gl: &Gl2) {
        gl.bind_texture(Gl2::TEXTURE_2D, Some(&self.0));
    }

    pub fn attach_framebuffer(&self, gl: &Gl2, attachment: u32) {
        gl.framebuffer_texture_2d(Gl2::FRAMEBUFFER, attachment, Gl2::TEXTURE_2D, Some(&self.0), 0)
    }
}
