use web_sys::WebGlVertexArrayObject as Vao;
use crate::error::WasmError;

pub struct DrawSprite {
    pub pos: (f32, f32),
}

impl DrawSprite {
    pub fn new(pos: (f32, f32)) -> Self {
        Self {
            pos, mesh, 
        }
    }
}
