use crate::graphics::WebGl;
use crate::render::Render;
use lazy_static::lazy_static;
use rask_engine::resources::{GetStore, ResourceTable};
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::mem;
use rask_wasm_shared::sprite::Frame;

lazy_static! {
    pub static ref RESOURCE_TABLE: ResourceTable = unsafe {
        let mut table =
            ResourceTable::from_memory(mem::RESOURCE_TABLE, mem::RESOURCE_TABLE_ELEMENT_COUNT);
        table.clear();
        table
    };
}

pub struct Context {
    render: Render<WebGl>,
}

impl Context {
    pub fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError> {
        Render::new(canvas).map(|render| Self { render })
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

/* pub fn context() -> &'static Context {
    unsafe { CONTEXT.as_ref().unwrap() }
} */
