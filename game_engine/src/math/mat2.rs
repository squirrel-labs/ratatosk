use std::ops;

use crate::math::Vec2;

/// A 2x2 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat2 {
    /// The elements of the matrix.
    /// [a, c,
    ///  b, d]
    data: [f32; 4],
}

impl ops::Add for Mat2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.data[0] + other.data[0],
            self.data[1] + other.data[1],
            self.data[2] + other.data[2],
            self.data[3] + other.data[3],
        )
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
        Self::new(
            self.data[0] - other.data[0],
            self.data[1] - other.data[1],
            self.data[2] - other.data[2],
            self.data[3] - other.data[3],
        )
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
        Self::new(-self.data[0], -self.data[1], -self.data[2], -self.data[3])
    }
}

impl ops::Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self::new(
            self.data[0] * scale,
            self.data[1] * scale,
            self.data[2] * scale,
            self.data[3] * scale,
        )
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

impl ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2::new(
            self.data[0] * other.x() + self.data[2] * other.y(),
            self.data[1] * other.x() + self.data[3] * other.y(),
        )
    }
}

impl ops::Mul for Mat2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.data[0] * other.data[0] + self.data[2] * other.data[1],
            self.data[1] * other.data[0] + self.data[3] * other.data[1],
            self.data[0] * other.data[2] + self.data[2] * other.data[3],
            self.data[1] * other.data[2] + self.data[3] * other.data[3],
        )
    }
}

impl ops::MulAssign for Mat2 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::Div<f32> for Mat2 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self::new(
            self.data[0] / scale,
            self.data[1] / scale,
            self.data[2] / scale,
            self.data[3] / scale,
        )
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
    /// Creates a new Mat2.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { data: [a, b, c, d] }
    }

    /// Creates a new Mat2 from two Vec2.
    pub fn from_vec2(v1: Vec2, v2: Vec2) -> Self {
        Self::new(v1.x(), v1.y(), v2.x(), v2.y())
    }

    /// Returns the zero matrix.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Returns a matrix that rotates by `angle`.
    pub fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(cos, sin, -sin, cos)
    }

    /// Returns a matrix that scales by `scale`.
    pub fn scaling(scale: f32) -> Self {
        Self::new(scale, 0.0, scale, 0.0)
    }

    /// Returns the transposed matrix.
    pub fn transpose(self) -> Self {
        Self::new(self.data[0], self.data[2], self.data[1], self.data[3])
    }
}
