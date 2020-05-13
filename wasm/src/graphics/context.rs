use super::render::Render;
use super::webgl::WebGl2;
use crate::error::ClientError;

pub struct Context {
    render: Render<WebGl2>,
}

impl Context {
    pub fn new() -> Result<Self, ClientError> {
        Render::new().map(|render| Self { render })
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        self.render.render()
    }
}

static mut CONTEXT: Option<Context> = None;

pub fn set_context(context: Context) {
    unsafe { CONTEXT = Some(context) }
}

pub fn context_mut() -> &'static mut Context {
    unsafe { CONTEXT.as_mut().unwrap() }
}
