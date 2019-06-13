use log::*;
use crate::error::WasmError;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as Gl;

pub struct GraphicsContext {
}

impl GraphicsContext {
    pub fn from_canvas(canvas: web_sys::OffscreenCanvas) -> Result<Self, WasmError> {
        /*debug!("canvas object usw.: {:?}", canvas);
        let canvas: web_sys::OffscreenCanvas = js_sys::Reflect::get(&canvas,
                                          &wasm_bindgen::JsValue::from_str("canvas"))
            .map_err(|_| WasmError::WebGl2ContextCreation(
                    format!("canvas object is not defined")))?
            .into();*/
        let context = canvas.get_context("webgl2")
            .map_err(|_| WasmError::WebGl2ContextCreation(
                    format!("context cration failed: getContext returned an exception")))?
            .ok_or_else(|| WasmError::WebGl2ContextCreation(
                    format!("context cration failed: getContext returned nothing")))?;
        let context = context
            .dyn_into::<Gl>()
            .map_err(|_| WasmError::WebGl2ContextCreation(
                    format!("context object is not a context")))?;

        context.clear_color(0.6, 0.0, 0.6, 1.0);
        context.clear(Gl::COLOR_BUFFER_BIT);
            
        Ok(Self {
        })
    }
}
