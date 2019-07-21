use std::{fmt, ops};

use packed_simd::{f32x2, shuffle};

use crate::math_simd::{Vec2, EPSILON};

#[derive(Clone, Copy)]
pub struct Mat2(f32x2, f32x2);

impl ops::Add for Mat2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::AddAssign for Mat2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl ops::Sub for Mat2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl ops::SubAssign for Mat2 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl ops::Neg for Mat2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl ops::Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self(self.0 * scale, self.1 * scale)
    }
}

impl ops::MulAssign<f32> for Mat2 {
    fn mul_assign(&mut self, scale: f32) {
        self.0 *= scale;
        self.1 *= scale;
    }
}

impl ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2(self.0 * other.0.extract(0) + self.1 * other.0.extract(1))
    }
}
/*
impl ops::Mul for Mat2 {
    type Output = Mat2;

    fn mul(self, other: Self) -> Self::Output {}
}

impl ops::MulAssign for Mat2 {
    fn mul_assign(&mut self, other: Self) {}
}
*/
impl ops::Div<f32> for Mat2 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self(self.0 / scale, self.1 / scale)
    }
}

impl ops::DivAssign<f32> for Mat2 {
    fn div_assign(&mut self, scale: f32) {
        self.0 /= scale;
        self.1 /= scale;
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs().lt(f32x2::splat(EPSILON)).all()
            && (self.1 - other.1).abs().lt(f32x2::splat(EPSILON)).all()
    }
}

impl Eq for Mat2 {}

impl Mat2 {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self(f32x2::new(a, c), f32x2::new(b, d))
    }

    pub fn from_vec2(v1: Vec2, v2: Vec2) -> Self {
        Self(v1.0, v2.0)
    }

    pub fn zero() -> Self {
        Self(f32x2::splat(0.0), f32x2::splat(0.0))
    }

    pub fn identity() -> Self {
        Self(f32x2::new(1.0, 0.0), f32x2::new(0.0, 1.0))
    }

    pub fn transpose(self) -> Self {
        Self(
            shuffle!(self.0, self.1, [0, 2]),
            shuffle!(self.0, self.1, [1, 3]),
        )
    }
}
