use crate::webhogg_exception::WebhoggException;

pub struct WebhoggApplication {
}

impl WebhoggApplication {
    pub fn new() -> Result<Self, WebhoggException> {
        Ok(Self {
        })
    }

    pub fn run(&mut self) -> Result<(), WebhoggException> {
        Ok(())
    }
}
