use webhogg_wasm_shared::error::ClientError;

pub struct Context {}

impl Context {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {})
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        log::info!("render!!");
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

pub fn context() -> &'static Context {
    unsafe { CONTEXT.as_ref().unwrap() }
}
