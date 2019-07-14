use std::{fmt, ops};

use packed_simd::f32x2;

#[derive(Clone, Copy)]
pub struct Vec2(f32x2);

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vec2 {{ x: {:?}, y: {:?} }}",
            self.0.extract(0),
            self.0.extract(1)
        )
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self(self.0 * scale)
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scale: f32) {
        self.0 *= scale;
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self(self.0 / scale)
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scale: f32) {
        self.0 /= scale;
    }
}

impl ops::Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

impl ops::DivAssign for Vec2 {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(f32x2::new(x, y))
    }

    pub fn zero() -> Self {
        Self(f32x2::splat(0.0))
    }

    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0).sum()
    }

    pub fn norm2(self) -> f32 {
        self.dot(self)
    }

    pub fn norm(self) -> f32 {
        self.norm2().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }
}
