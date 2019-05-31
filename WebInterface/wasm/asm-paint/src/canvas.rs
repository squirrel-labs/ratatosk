use web_sys;
use web_sys::{WebGl2RenderingContext};
use wasm_bindgen::JsCast;
use crate::shader::Shaders;

pub struct Canvas {
    element: web_sys::HtmlCanvasElement,
    ctx: WebGl2RenderingContext,
    shaders: Shaders,
}

impl Canvas {
    pub fn new(element: web_sys::Element) -> Option<Self> {
        let element: web_sys::HtmlCanvasElement =
            element.dyn_into::<web_sys::HtmlCanvasElement>()
            .ok()?;
        debug!("create webgl2 context");
        let ctx = element.get_context("webgl2").ok()??
            .dyn_into::<WebGl2RenderingContext>().ok()?;
        info!("created webgl2 context successfully");
        Some(Self {
            element, ctx,
            shaders: Shaders::new(),
        })
    }

    pub fn init(&mut self) -> Result<(), ()> {
        self.shaders.init(&self.ctx).map_err(|_|())?;
        self.ctx.clear_color(1.0, 0.2, 1.0, 1.0);
        self.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        Ok(())
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        self.shaders.remove(&self.ctx);
    }
}
