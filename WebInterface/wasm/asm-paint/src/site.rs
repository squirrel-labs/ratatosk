use web_sys;
use crate::canvas::Canvas;

pub struct Site {
    window: web_sys::Window,
    document: web_sys::Document,
}

impl Site {
    pub fn from_current() -> Option<Self> {
        let window = web_sys::window()
            .or_else(|| {error!("unable to query window"); None})?;
        let document = window.document()
            .or_else(|| {error!("unable to query document"); None})?;
        Some(Self { 
            window, document
        })
    }

    pub fn create_canvas(&self) -> Option<Canvas> {
        debug!("gain canvas element");
        let element = self.document.get_element_by_id("canvas")
            .or_else(|| {error!("could not gain canvas element"); None})?;
        Canvas::new(element)
            .or_else(|| {error!("could not create a webgl2 canvas.
 Your browser doesn't seem to support webgl2"); None})
    }
}
