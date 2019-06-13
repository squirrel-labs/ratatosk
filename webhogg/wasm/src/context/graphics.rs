use log::*;
use crate::error::WasmError;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GlContext;

use super::webgl::{Color4, WebGl2};

pub struct GraphicsContext {
    gl: WebGl2,
    frame_nr: u64,
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
            
        Ok(Self {
            gl, frame_nr: 0,
        })
    }

    pub fn update(&mut self) -> Result<(), WasmError> {
        let light = 0.5;
        let speed = 60.0;

        let a = (self.frame_nr as f32) / speed;
        let a = f32::abs(f32::sin(a));
        let b = f32::abs(f32::cos(a));
        let (a, b) = (a * light, b * light);

        self.gl.clear(Color4::new(a, light - a, b, 1.0));

        self.frame_nr += 1;

        Ok(())
    }
}
