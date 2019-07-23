use std::ops;

use crate::math::Vec2;

/// An axis-aligned box.
#[derive(Clone, Copy, Debug)]
pub struct AABox {
    pub pos: Vec2,
    /// the size may not be smaller than zero
    pub size: Vec2,
}

impl ops::Add<Vec2> for AABox {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            size: self.size,
        }
    }
}

impl ops::AddAssign<Vec2> for AABox {
    fn add_assign(&mut self, other: Vec2) {
        self.pos += other
    }
}

impl ops::Sub<Vec2> for AABox {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            pos: self.pos - other,
            size: self.size,
        }
    }
}

impl ops::SubAssign<Vec2> for AABox {
    fn sub_assign(&mut self, other: Vec2) {
        self.pos -= other
    }
}

impl std::cmp::PartialEq for AABox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.size == other.size
    }
}

impl std::cmp::Eq for AABox {}

/// A rotated box.
#[derive(Clone, Copy, Debug)]
pub struct RBox {
    /// The origin.
    pub pos: Vec2,
    /// Vector1
    pub v1: Vec2,
    /// Vector2: this Vector has to always be orthogonal to v1
    pub v2: Vec2,
}

impl RBox {
    /// Creates a new rotated box from a position, an orientation and a width.
    pub fn new(pos: Vec2, orientation: Vec2, width: f32) -> Self {
        let scale = width / orientation.norm();
        let orth = Vec2::new(orientation.x(), -orientation.y()) / scale;
        Self {
            pos,
            v1: orientation,
            v2: orth,
        }
    }
}

impl ops::Add<Vec2> for RBox {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            v1: self.v1,
            v2: self.v2,
        }
    }
}

impl ops::AddAssign<Vec2> for RBox {
    fn add_assign(&mut self, other: Vec2) {
        self.pos += other
    }
}

impl ops::Sub<Vec2> for RBox {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            pos: self.pos - other,
            v1: self.v1,
            v2: self.v2,
        }
    }
}

impl ops::SubAssign<Vec2> for RBox {
    fn sub_assign(&mut self, other: Vec2) {
        self.pos -= other
    }
}

impl std::cmp::PartialEq for RBox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.v1 == other.v1 && self.v2 == other.v2
    }
}

impl std::cmp::Eq for RBox {}
