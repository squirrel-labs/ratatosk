pub mod matrix_mul;
pub mod matrix;
mod shader;
mod draw_sprite;
mod webgl;
pub mod graphics;
pub mod logic;

use graphics::GraphicsContext;
use logic::LogicContext;

//static GTX: usize = Box::into_raw(Box::new());
//static LTX: usize = Box::into_raw(Box::new());

/*pub fn get_graphics() -> &'static mut GraphicsContext {
    unsafe { &mut *(GTX.unwrap() as *mut GraphicsContext) }
}

pub fn get_logic() -> &'static mut LogicContext {
    unsafe { &mut *(LTX.unwrap() as *mut LogicContext) }
}

pub fn set_graphics(gtx: GraphicsContext) {
    unsafe {
        GTX
    }
}

pub fn set_logic(ltx: LogicContext) {
    LTX = Some(Box::into_raw(Box::new(ltx)) as usize)
}*/
