pub struct Matrix {
    pub raw: [f32; 9],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            raw: [1.0, 0.0, 0.0,
                  0.0, 1.0, 0.0,
                  0.0, 0.0, 1.0]
        }
    }
    
    /// multiply with scale matrix
    pub fn scale(self, s: (f32, f32)) -> Self {
        self * Matrix {
            raw: [s.0, 0.0, 0.0,
                  0.0, s.1, 0.0,
                  0.0, 0.0, 1.0]
        }
    }

    /// multiply with rotation matrix
    pub fn rot(self, angle: f32) -> Self {
        self * Matrix {
            raw: [f32::cos(angle), -f32::sin(angle), 0.0,
                  f32::sin(angle), f32::cos(angle), 0.0,
                  0.0, 0.0, 1.0]
        }
    }

    pub fn translate(self, d: (f32, f32)) -> Self {
        self * Matrix {
            raw: [1.0, 0.0, d.0,
                  0.0, 1.0, d.1,
                  0.0, 0.0, 1.0]
        }
    }
}
