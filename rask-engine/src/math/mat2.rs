use core::convert;
use core::ops;

use crate::math::{Vec2, EPSILON};

/// A 2x2 matrix with `f32` elements.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mat2 {
    // The elements of the matrix.
    // (a c)
    // (b d)
    data: [f32; 4],
}

impl ops::Add for Mat2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.data[0] + other.data[0],
            self.data[2] + other.data[2],
            self.data[1] + other.data[1],
            self.data[3] + other.data[3],
        )
    }
}

impl ops::AddAssign for Mat2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl ops::Sub for Mat2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.data[0] - other.data[0],
            self.data[2] - other.data[2],
            self.data[1] - other.data[1],
            self.data[3] - other.data[3],
        )
    }
}

impl ops::SubAssign for Mat2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl ops::Neg for Mat2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.data[0], -self.data[2], -self.data[1], -self.data[3])
    }
}

impl ops::Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self::new(
            self.data[0] * scale,
            self.data[2] * scale,
            self.data[1] * scale,
            self.data[3] * scale,
        )
    }
}

impl ops::Mul<Mat2> for f32 {
    type Output = Mat2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Mat2 {
    fn mul_assign(&mut self, scale: f32) {
        *self = *self * scale
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
            self.data[0] * other.data[2] + self.data[2] * other.data[3],
            self.data[1] * other.data[0] + self.data[3] * other.data[1],
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
            self.data[2] / scale,
            self.data[1] / scale,
            self.data[3] / scale,
        )
    }
}

impl ops::DivAssign<f32> for Mat2 {
    fn div_assign(&mut self, scale: f32) {
        *self = *self / scale
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.data[0] - other.data[0]) < EPSILON
            && f32::abs(self.data[2] - other.data[2]) < EPSILON
            && f32::abs(self.data[1] - other.data[1]) < EPSILON
            && f32::abs(self.data[3] - other.data[3]) < EPSILON
    }
}

impl Eq for Mat2 {}

impl Mat2 {
    /// Creates a new `Mat2` of the form:
    /// (a b)
    /// (c d)
    pub const fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { data: [a, c, b, d] }
    }

    /// Creates a new `Mat2` from two column `Vec2`s.
    pub const fn from_vec2(v1: Vec2, v2: Vec2) -> Self {
        Self::new(v1.x(), v2.x(), v1.y(), v2.y())
    }

    /// Returns the zero matrix.
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Returns the identity matrix.
    pub const fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Returns a matrix that rotates by `angle`.
    pub fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(cos, -sin, sin, cos)
    }

    /// Returns a matrix that scales by `scale_x` and `scale_y`.
    pub const fn scaling(scale_x: f32, scale_y: f32) -> Self {
        Self::new(scale_x, 0.0, 0.0, scale_y)
    }

    /// Returns the transposed matrix.
    pub const fn transpose(self) -> Self {
        Self::new(self.data[0], self.data[1], self.data[2], self.data[3])
    }
}

impl convert::AsRef<[f32; 4]> for Mat2 {
    fn as_ref(&self) -> &[f32; 4] {
        &self.data
    }
}
