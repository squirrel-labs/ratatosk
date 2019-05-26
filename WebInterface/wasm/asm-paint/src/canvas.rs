use web_sys;
use web_sys::{WebGl2RenderingContext};
use wasm_bindgen::JsCast;

pub struct Canvas {
    element: web_sys::HtmlCanvasElement,
    ctx: WebGl2RenderingContext,
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
            element, ctx
        })
    }

    pub fn render(&self) {
        info!("do a barrel roll");
    }
}
