use web_sys;

use crate::webhogg_exception::WebhoggException;

pub struct Page {
    window: web_sys::Window,
    document: web_sys::Document,
}

impl Page {
    pub fn obtain() -> Result<Self, WebhoggException> {
        let window = web_sys::window()
            .ok_or(WebhoggException::DomError("could not obtain window".to_string()))?;
        let document = window.document()
            .ok_or(WebhoggException::DomError("could not obtain document".to_string()))?;
        debug!("initialised page");
        Ok(Self {
            window,
            document,
        })
    }

    pub fn get_element(&self, id: &str) -> Option<web_sys::Element> {
        self.document.get_element_by_id(id)
    }
}
