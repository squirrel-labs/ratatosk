use std::ops;

/// A 2x2 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat2 {
    data: [f32; 4],
}

