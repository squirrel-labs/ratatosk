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

/// # Safety
/// This function is not thread safe.
pub unsafe fn set_context(context: Context) -> &'static mut Context {
    CONTEXT = Some(context);
    CONTEXT.as_mut().unwrap()
}

/// # Safety
/// This function is not thread safe.
pub unsafe fn context_mut() -> Option<&'static mut Context> {
    CONTEXT.as_mut()
}
