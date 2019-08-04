use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as Gl2;
use webhogg_wasm_shared::error::ClientError;

pub trait GraphicsApi: Sized {
    fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError>;
}

pub struct WebGl {
    gl: Gl2,
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
        gl.clear_color(1.0, 0.0, 0.0, 1.0);
        gl.clear(Gl2::COLOR_BUFFER_BIT);
        Ok(WebGl { canvas, gl })
    }
}
