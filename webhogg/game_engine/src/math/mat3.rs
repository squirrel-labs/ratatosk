use std::ops;

/// A 3x3 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    data: [f32; 9],
}

