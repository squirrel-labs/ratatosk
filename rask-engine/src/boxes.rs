//! The boxes module exposes AABox (axis-aligned box) and RBox (rotated box).
//! These are used to model objects.

use core::ops;

use crate::math::{Mat2, Mat3, Vec2, Vec3};

/// An axis-aligned box.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AABox {
    /// The position of the box.
    pub pos: Vec2,
    /// The size, both components must be greater than zero.
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

/// A rotated box.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    // v1 has the same direction as v2 rotated to the left by 90°
    pub fn new(pos: Vec2, orientation: Vec2, height: f32) -> Self {
        let scale = height / orientation.norm();
        let orth = Vec2::new(-orientation.y(), orientation.x()) * scale;
        Self {
            pos,
            v1: orth,
            v2: orientation,
        }
    }

    /// Returns a copy of the Box with a normalized internal layout.
    /// "Normal Constraints":
    /// * `pos` is the lowest corner
    ///     (the `y` components of `v1` and `v2` are non negative)
    /// * `v1` is left of `v2`
    ///     (the `x` component of `v2` is greater or equal to the `x` component of `v1`)
    /// * `v1` and `v2` are at a right angle to each other
    /// TODO: force normalization form for every RBox
    pub fn as_normal_form(&self) -> Self {
        let (pos, v1, v2, b) = if self.v2.y() < 0.0 {
            if self.v1.y() < 0.0 {
                (
                    self.pos + self.v1 + self.v2,
                    -self.v1,
                    -self.v2,
                    self.v1.x() >= 0.0,
                )
            } else {
                (self.pos + self.v2, -self.v2, self.v1, self.v2.x() >= 0.0)
            }
        } else if self.v1.y() < 0.0 {
            (self.pos + self.v1, self.v2, -self.v1, self.v1.x() < 0.0)
        } else {
            (self.pos, self.v1, self.v2, self.v1.x() < 0.0)
        };
        let (v1, v2) = if b { (v1, v2) } else { (v2, v1) };
        Self { pos, v1, v2 }
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

impl From<AABox> for RBox {
    fn from(aabox: AABox) -> Self {
        Self {
            pos: aabox.pos,
            v1: Vec2::new(aabox.size.x(), 0.0),
            v2: Vec2::new(0.0, aabox.size.y()),
        }
    }
}

impl From<&spine::skeleton::srt::SRT> for RBox {
    fn from(srt: &spine::skeleton::srt::SRT) -> RBox {
        let pos = srt.transform([-1.0, -1.0]).into();
        let v1 = Vec2::from(srt.transform([1.0, -1.0])) - pos;
        let v2 = Vec2::from(srt.transform([-1.0, 1.0])) - pos;
        RBox { pos, v1, v2 }
    }
}

impl From<&Mat3> for RBox {
    fn from(mat: &Mat3) -> RBox {
        let pos = Vec2::from(*mat * Vec3::new(-1.0, -1.0, 1.0));
        let v1 = Mat2::from(mat) * Vec2::new(0.0, 2.0);
        let v2 = Mat2::from(mat) * Vec2::new(2.0, 0.0);
        RBox { pos, v1, v2 }
    }
}
