use web_sys::WebGl2RenderingContext as WebGl2;
use wasm_bindgen::JsCast;
use crate::webhogg_exception::WebhoggException;
use crate::page::Page;

pub struct Canvas {
    ctx: WebGl2,
}

impl Canvas {
    pub fn from_existing(id: &str, page: &Page) -> Result<Self, WebhoggException> {
        let canvas_element = page.get_element(id)
            .ok_or(WebhoggException::DomError(
                    "could not obtain canvas element (id=canvas)"
                    .to_string()))?;
        let canvas_element = canvas_element
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| WebhoggException::DomError(
                    "id=canvas is not a canvas element".to_string()))?;
        debug!("successfully obtained canvas element");
        let ctx = canvas_element.get_context("webgl2")
            .map_err(|_| WebhoggException::WebGlContextError(
                    "obtained invalid webgl2 context js value".to_string()))?
            .ok_or(WebhoggException::WebGlContextError(
                    "could not obtaine webgl2 context".to_string()))?
            .dyn_into::<WebGl2>()
            .map_err(|_| WebhoggException::WebGlContextError(
                    "obtained invalid webgl2 context js object".to_string()))?;
        debug!("successfully obtained webgl2 context");
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
