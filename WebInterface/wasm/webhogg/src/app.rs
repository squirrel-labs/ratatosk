use crate::webhogg_exception::WebhoggException;
use crate::page::Page;
use crate::canvas::Canvas;

pub(crate) struct WebhoggApplication {
    page: Page,
    canvas: Canvas,
}

impl WebhoggApplication {
    pub fn new() -> Result<Self, WebhoggException> {
        let page = Page::obtain()?;
        let canvas = Canvas::from_existing("canvas", &page)?;
        Ok(Self {
            page, canvas,
        })
    }

    pub fn run(self) -> Result<(), WebhoggException> {
        Ok(())
    }
}
