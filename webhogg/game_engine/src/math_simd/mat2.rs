use std::{fmt, ops};

use packed_simd::{f32x4, shuffle};

use crate::math_simd::{EPSILON, Vec2};

pub struct Mat2(f32x4);

impl ops::Add for Mat2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl ops::AddAssign for Mat2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl ops::Sub for Mat2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl ops::SubAssign for Mat2 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl ops::Neg for Mat2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl ops::Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self(self.0 * scale)
    }
}

impl ops::MulAssign<f32> for Mat2 {
    fn mul_assign(&mut self, scale: f32) {
        self.0 *= scale;
    }
}
/*
impl ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {}
}

impl ops::MulAssign<Vec2> for Mat2 {
    fn mul_assign(&mut self, other: Vec2) {}
}

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
        Self(self.0 / scale)
    }
}

impl ops::DivAssign<f32> for Mat2 {
    fn div_assign(&mut self, scale: f32) {
        self.0 /= scale;
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs().lt(f32x4::splat(EPSILON)).all()
    }
}

impl Eq for Mat2 {}

impl Mat2 {
    pub fn zero() -> Self {
        Self(f32x4::splat(0.0))
    }

    pub fn identity() -> Self {
        Self(f32x4::new(1.0, 0.0, 0.0, 1.0))
    }

    pub fn transpose(self) -> Self {
        Self(shuffle!(self.0, [0, 2, 1, 3]))
    }
}
