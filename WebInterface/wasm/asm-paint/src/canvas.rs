use web_sys;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use wasm_bindgen::JsCast;

pub struct Canvas {
    element: web_sys::HtmlCanvasElement,
    ctx: WebGlRenderingContext,
}

impl Canvas {
    pub fn new(element: web_sys::Element) -> Option<Self> {
        let element: web_sys::HtmlCanvasElement =
            element.dyn_into::<web_sys::HtmlCanvasElement>()
            .ok()?;
        debug!("create webgl2 context");
        error!("'{:#?}'", element.get_context("webgl2").ok()??.dyn_into::<WebGlRenderingContext>());
        let ctx = element.get_context("webgl2").ok()??
            .dyn_into::<WebGlRenderingContext>().ok()?;
        Some(Self {
            element, ctx
        })
    }

    pub fn render(&self) {
        info!("rich kidd");
    }
}
