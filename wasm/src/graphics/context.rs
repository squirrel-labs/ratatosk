use super::render::Render;
use crate::error::ClientError;

pub struct Context {
    //render: Render<WebGl>,
}

impl Context {
    pub fn new() -> Result<Self, ClientError> {
        //Render::new().map(|render| Self { render })
        Ok(Self {})
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        //self.render.render()
        Ok(())
    }
}

static mut CONTEXT: Option<Context> = None;

pub fn set_context(context: Context) {
    unsafe { CONTEXT = Some(context) }
}

pub fn context_mut() -> &'static mut Context {
    unsafe { CONTEXT.as_mut().unwrap() }
}
