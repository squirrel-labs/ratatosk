use log::*;
use crate::error::WasmError;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as Gl;

pub struct GraphicsContext {
    gl: Gl,
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
            .dyn_into::<Gl>()
            .map_err(|_| WasmError::WebGl2ContextCreation(
                    format!("context object is not a context")))?;
            
        Ok(Self {
            gl: context,
            frame_nr: 0,
        })
    }

    pub fn update(&mut self) -> Result<(), WasmError> {
        let light = 0.5;

        let a = (self.frame_nr as f32) / 60.0;
        let a = f32::abs(f32::sin(a));
        let b = f32::abs(f32::cos(a));
        let (a, b) = (a * light, b * light);

        self.gl.clear_color(a, light - a, b, 1.0);
        self.gl.clear(Gl::COLOR_BUFFER_BIT);

        self.frame_nr += 1;

        Ok(())
    }
}
