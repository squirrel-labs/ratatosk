//! The boxes module exposes AABox (axis-aligned box) and RBox (rotated box).
//! These are used to model objects.

use core::ops;

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

impl PartialEq for AABox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.size == other.size
    }
}

impl Eq for AABox {}

/// A rotated box.
#[derive(Clone, Copy, Debug)]
pub struct RBox {
    /// The origin.
    pub pos: Vec2,
    /// Vector 1.
    pub v1: Vec2,
    /// Vector 2: This vector always has to be orthogonal to v1.
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

impl PartialEq for RBox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.v1 == other.v1 && self.v2 == other.v2
    }
}

impl From<&spine::skeleton::SRT> for RBox {
    fn from(srt: &spine::skeleton::SRT) -> RBox {
        use crate::math::Vec3;
        let mat3 = super::math::Mat3::from_nested_arr(&srt.to_matrix3())
            .expect("Fatal error, spine matrix is not 3x3");
        let pos = (mat3 * Vec3::new(-1f32, 1f32, 1f32)).to_vec2();
        let v1 = pos - (mat3 * Vec3::new(-1f32, -1f32, 1f32)).to_vec2();
        let v2 = pos - (mat3 * Vec3::new(1f32, 1f32, 1f32)).to_vec2();
        RBox { pos, v1, v2 }
    }
}

impl Eq for RBox {}
