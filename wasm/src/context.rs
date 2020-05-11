use crate::error::ClientError;
use crate::mem;
use crate::render::Render;
use lazy_static::lazy_static;
use rask_engine::resources::ResourceTable;

lazy_static! {
    pub static ref RESOURCE_TABLE: ResourceTable = unsafe {
        ResourceTable::from_memory(
            mem::MEM_ADDRS.read().resource_table as usize,
            mem::RESOURCE_TABLE_ELEMENT_COUNT as usize,
        )
    };
}

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
