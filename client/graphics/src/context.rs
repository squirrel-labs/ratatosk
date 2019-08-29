use crate::graphics::WebGl;
use crate::render::Render;
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::sprite::{Animation, Frame};

pub struct Context {
    render: Render<WebGl>,
    animations: Vec<Animation>,
}

impl Context {
    pub fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError> {
        Render::new(canvas).map(|render| Self {
            render,
            animations: vec![Animation::new(vec![Frame::new(vec![
                rask_engine::math::Mat3::identity(),
            ])])],
        })
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        self.render.render(&self.animations)
    }
}

static mut CONTEXT: Option<Context> = None;

pub fn set_context(context: Context) {
    unsafe { CONTEXT = Some(context) }
}

pub fn context_mut() -> &'static mut Context {
    unsafe { CONTEXT.as_mut().unwrap() }
}

/* pub fn context() -> &'static Context {
    unsafe { CONTEXT.as_ref().unwrap() }
} */
