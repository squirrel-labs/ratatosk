use std::ops;

use crate::math::Vec3;

/// A 3x3 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    /// The elements of the matrix.
    /// [a, b, c
    ///  d, e, f
    ///  g, h, i]
    data: [f32; 9],
}

impl ops::Add for Mat3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
                self.data[3] + other.data[3],
                self.data[4] + other.data[4],
                self.data[5] + other.data[5],
                self.data[6] + other.data[6],
                self.data[7] + other.data[7],
                self.data[8] + other.data[8],
            ],
        }
    }
}

impl ops::AddAssign for Mat3 {
    fn add_assign(&mut self, other: Self) {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
        self.data[2] += other.data[2];
        self.data[3] += other.data[3];
        self.data[4] += other.data[4];
        self.data[5] += other.data[5];
        self.data[6] += other.data[6];
        self.data[7] += other.data[7];
        self.data[8] += other.data[8];
    }
}

impl ops::Sub for Mat3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
                self.data[3] - other.data[3],
                self.data[4] - other.data[4],
                self.data[5] - other.data[5],
                self.data[6] - other.data[6],
                self.data[7] - other.data[7],
                self.data[8] - other.data[8],
            ],
        }
    }
}

impl ops::SubAssign for Mat3 {
    fn sub_assign(&mut self, other: Self) {
        self.data[0] -= other.data[0];
        self.data[1] -= other.data[1];
        self.data[2] -= other.data[2];
        self.data[3] -= other.data[3];
        self.data[4] -= other.data[4];
        self.data[5] -= other.data[5];
        self.data[6] -= other.data[6];
        self.data[7] -= other.data[7];
        self.data[8] -= other.data[8];
    }
}

impl ops::Neg for Mat3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            data: [
                -self.data[0],
                -self.data[1],
                -self.data[2],
                -self.data[3],
                -self.data[4],
                -self.data[5],
                -self.data[6],
                -self.data[7],
                -self.data[8],
            ],
        }
    }
}

impl ops::Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self {
            data: [
                self.data[0] * scale,
                self.data[1] * scale,
                self.data[2] * scale,
                self.data[3] * scale,
                self.data[4] * scale,
                self.data[5] * scale,
                self.data[6] * scale,
                self.data[7] * scale,
                self.data[8] * scale,
            ],
        }
    }
}

impl ops::MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, scale: f32) {
        self.data[0] *= scale;
        self.data[1] *= scale;
        self.data[2] *= scale;
        self.data[3] *= scale;
        self.data[4] *= scale;
        self.data[5] *= scale;
        self.data[6] *= scale;
        self.data[7] *= scale;
        self.data[8] *= scale;
    }
}
/*
impl ops::Mul<Vec3> for Mat3 {}

impl ops::MulAssign<Vec3> for Mat3 {}

impl ops::Mul for Mat3 {}

impl ops::MulAssign for Mat3 {}

impl ops::Div<f32> for Mat3 {}

impl ops::DivAssign<f32> for Mat3 {}
*/
impl Mat3 {
    pub fn zero() -> Self {
        Self {
            data: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn identity() -> Self {
        Self {
            data: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            data: [
                self.data[0],
                self.data[3],
                self.data[6],
                self.data[1],
                self.data[4],
                self.data[7],
                self.data[2],
                self.data[5],
                self.data[8],
            ],
        }
    }
}
