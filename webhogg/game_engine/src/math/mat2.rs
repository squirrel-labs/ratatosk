use std::ops;

use crate::math::Vec2;

/// A 2x2 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat2 {
    /// The elements of the matrix.
    /// [a, b,
    ///  c, d]
    data: [f32; 4],
}

impl ops::Add for Mat2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
                self.data[3] + other.data[3],
            ],
        }
    }
}

impl ops::AddAssign for Mat2 {
    fn add_assign(&mut self, other: Self) {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
        self.data[2] += other.data[2];
        self.data[3] += other.data[3];
    }
}

impl ops::Sub for Mat2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
                self.data[3] - other.data[3],
            ],
        }
    }
}

impl ops::SubAssign for Mat2 {
    fn sub_assign(&mut self, other: Self) {
        self.data[0] -= other.data[0];
        self.data[1] -= other.data[1];
        self.data[2] -= other.data[2];
        self.data[3] -= other.data[3];
    }
}

impl ops::Neg for Mat2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            data: [-self.data[0], -self.data[1], -self.data[2], -self.data[3]],
        }
    }
}

impl ops::Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self {
            data: [
                self.data[0] * scale,
                self.data[1] * scale,
                self.data[2] * scale,
                self.data[3] * scale,
            ],
        }
    }
}

impl ops::MulAssign<f32> for Mat2 {
    fn mul_assign(&mut self, scale: f32) {
        self.data[0] *= scale;
        self.data[1] *= scale;
        self.data[2] *= scale;
        self.data[3] *= scale;
    }
}
/*
impl ops::Mul<Vec2> for Mat2 {}

impl ops::MulAssign<Vec2> for Mat2 {}

impl ops::Mul for Mat2 {}

impl ops::MulAssign for Mat2 {}
*/
impl ops::Div<f32> for Mat2 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self {
            data: [
                self.data[0] / scale,
                self.data[1] / scale,
                self.data[2] / scale,
                self.data[3] / scale,
            ],
        }
    }
}

impl ops::DivAssign<f32> for Mat2 {
    fn div_assign(&mut self, scale: f32) {
        self.data[0] /= scale;
        self.data[1] /= scale;
        self.data[2] /= scale;
        self.data[3] /= scale;
    }
}

impl Mat2 {
    pub fn zero() -> Self {
        Self {
            data: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn identity() -> Self {
        Self {
            data: [1.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            data: [self.data[0], self.data[2], self.data[1], self.data[3]],
        }
    }
}
