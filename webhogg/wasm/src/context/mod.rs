pub mod graphics;
pub mod logic;

use graphics::GraphicsContext;
use logic::LogicContext;

static mut GTX: Option<GraphicsContext> = None;
static mut LTX: Option<LogicContext> = None;

pub fn get_graphics() -> &'static mut GraphicsContext {
    unsafe { GTX.as_mut().unwrap() }
}

pub fn get_logic() -> &'static mut LogicContext {
    unsafe { LTX.as_mut().unwrap() }
}

pub fn set_graphics(gtx: GraphicsContext) {
    unsafe { GTX = Some(gtx) }
}

pub fn set_logic(ltx: LogicContext) {
    unsafe { LTX = Some(ltx) }
}
