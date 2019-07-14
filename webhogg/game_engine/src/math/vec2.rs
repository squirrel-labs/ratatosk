use std::cmp::Ordering;
use std::ops;

use crate::math::EPSILON;

/// A 2-dimensional euclidean vector.
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    /// The x coordinate.
    x: f32,
    /// The y coordinate.
    y: f32,
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self::new(self.x * scale, self.y * scale)
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self::new(self.x / scale, self.y / scale)
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scale: f32) {
        self.x /= scale;
        self.y /= scale;
    }
}

impl ops::Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(self.x / other.x, self.y / other.y)
    }
}

impl ops::DivAssign for Vec2 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.x <= other.x && self.y <= other.y {
            Some(Ordering::Less)
        } else if self.x >= other.x && self.y >= other.y {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < EPSILON && f32::abs(self.y - other.y) < EPSILON
    }
}

impl Eq for Vec2 {}

impl Vec2 {
    /// Creates a new Vec2 from x and y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the zero vector.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Returns the x coordinate.
    pub fn x(self) -> f32 {
        self.x
    }

    /// Returns the y coordinate.
    pub fn y(self) -> f32 {
        self.y
    }

    /// Returns the dot product.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the square of the euclidean norm of the vector.
    pub fn norm2(self) -> f32 {
        self.dot(self)
    }

    /// Returns the euclidean norm of the vector.
    pub fn norm(self) -> f32 {
        f32::hypot(self.x, self.y)
    }

    /// Returns a normalized version of the vector, that is, a vector that points in the same direction, but has norm 1.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }
}
