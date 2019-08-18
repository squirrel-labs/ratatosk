use std::cmp::Ordering;
use std::{fmt, ops};

use packed_simd::f32x2;

use crate::math_simd::EPSILON;

#[derive(Clone, Copy)]
pub struct Vec2(pub(super) f32x2);

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

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.0.le(other.0).all() {
            Some(Ordering::Less)
        } else if self.0.ge(other.0).all() {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs().lt(f32x2::splat(EPSILON)).all()
    }
}

impl Eq for Vec2 {}

impl Vec2 {
    /// Creates a new Vec2 from x and y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self(f32x2::new(x, y))
    }

    /// Returns the zero vector.
    pub fn zero() -> Self {
        Self(f32x2::splat(0.0))
    }

    /// Returns the x coordinate.
    pub fn x(self) -> f32 {
        self.0.extract(0)
    }

    /// Returns the y coordinate.
    pub fn y(self) -> f32 {
        self.0.extract(1)
    }

    /// Returns the dot product.
    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0).sum()
    }

    /// Returns the square of the euclidean norm of the vector.
    pub fn norm2(self) -> f32 {
        self.dot(self)
    }

    /// Returns the euclidean norm of the vector.
    pub fn norm(self) -> f32 {
        self.norm2().sqrt()
    }

    /// Returns a normalized version of the vector, that is, a vector that points in the same direction, but has norm 1.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }
}
