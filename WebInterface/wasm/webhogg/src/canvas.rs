use web_sys::WebGl2RenderingContext as WebGl2;
use wasm_bindgen::JsCast;
use crate::webhogg_exception::WebhoggException;
use web_sys::OffscreenCanvas as ECanvas;

pub struct Canvas {
    ctx: WebGl2,
}

impl Canvas {
    pub fn from_existing(canvas: &ECanvas) -> Result<Self, WebhoggException> {
        info!("messages lol");
        let ctx = canvas.get_context("webgl2")
            .map_err(|x| WebhoggException::WebGlContextError(
                    format!("obtained invalid webgl2 context js value: {:?}", x)))?
            .ok_or(WebhoggException::WebGlContextError(
                    "could not obtaine webgl2 context".to_string()))?
            .dyn_into::<WebGl2>()
            .map_err(|_| WebhoggException::WebGlContextError(
                    "obtained invalid webgl2 context js object".to_string()))?;
        info!("successfully obtained webgl2 context");
        ctx.clear_color(0.6, 0.0, 0.6, 1.0);
        ctx.clear(WebGl2::COLOR_BUFFER_BIT);
        Ok(Self {
            ctx,
        })
    }

    pub fn gl<'a>(&'a self) -> &'a WebGl2 {
        &self.ctx
    }

    pub fn gl_mut<'a>(&'a mut self) -> &'a mut WebGl2 {
        &mut self.ctx
    }
}
