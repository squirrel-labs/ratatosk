use super::matrix::Matrix;
use crate::error::WasmError;
use web_sys::WebGlVertexArrayObject as Vao;

pub struct DrawSprite {
    pub pos: (f32, f32),
    pub transform: Matrix,
}

impl DrawSprite {
    pub fn new(pos: (f32, f32), transform: Matrix) -> Self {
        Self { pos, transform }
    }
}
