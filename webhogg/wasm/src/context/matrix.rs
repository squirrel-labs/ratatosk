#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub raw: [f32; 9],
}

impl Matrix {
    pub fn new(
            a: f32, b: f32, c: f32,
            d: f32, e: f32, f: f32,
            g: f32, h: f32, i: f32) -> Self {
        Self {
            raw: [a, d, g,
                  b, e, h,
                  c, f, i]
        }
    }
}

impl Matrix {
    pub fn identity() -> Self {
        Matrix::new(1.0, 0.0, 0.0,
                    0.0, 1.0, 0.0,
                    0.0, 0.0, 1.0)
    }
    
    /// multiply with scale matrix
    pub fn scale(self, s: (f32, f32)) -> Self {
        Matrix::new(s.0, 0.0, 0.0,
                    0.0, s.1, 0.0,
                    0.0, 0.0, 1.0)
            * self
    }

    /// multiply with rotation matrix
    pub fn rot(self, angle: f32) -> Self {
        Matrix::new(f32::cos(angle), f32::sin(angle), 0.0,
                    -f32::sin(angle), f32::cos(angle), 0.0,
                    0.0, 0.0, 1.0)
            * self
    }

    pub fn translate(self, d: (f32, f32)) -> Self {
        Matrix::new(1.0, 0.0, d.0,
                    0.0, 1.0, d.1,
                    0.0, 0.0, 1.0)
            * self
    }
}
