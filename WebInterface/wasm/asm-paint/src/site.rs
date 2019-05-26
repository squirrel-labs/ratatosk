use web_sys;

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
}
