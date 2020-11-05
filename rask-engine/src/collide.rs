//! The collide module provides the Collide trait for objects that can collide along with several
//! implementations for various types.

use crate::boxes::{AABox, RBox};
use crate::math::{Vec2, EPSILON};
use spine::skeleton::srt::SRT;

// For information on the SAT, see: http://www.dyn4j.org/2010/01/sat/.

/// A trait for objects that can collide with other objects.
pub trait Collide<Rhs = Self> {
    /// calculate the penetration depth of the collision after applying the velocity,
    /// if there was any.
    /// `dv` is the difference between the two velocities
    fn collide_after(&self, other: &Rhs, dv: Vec2) -> Option<f32>;
}

/// A trait for common objects to be collidable with other common objects.
pub trait Collidable: Collide<RBox> + Collide<SRT> + Collide<AABox> + Collide<Vec2> {}

fn left_under(v1: Vec2, v2: Vec2) -> bool {
    v1.x() < v2.x() && v1.y() < v2.y()
}

macro_rules! impl_collide {
    (for {$A:ident} $item:item) => {
        impl Collide for $A {
            $item
        }
        //TODO:
        //impl Collidable for $A {}
    };
    (for {$A:ident, $B:ident} $item:item) => {
        impl Collide<$B> for $A {
            $item
        }
        impl Collide<$A> for $B {
            fn collide_after(&self, other: &$A, dv: Vec2) -> Option<f32> {
                other.collide_after(self, -dv).map(|r| -r)
            }
        }
    };
}

impl_collide!(for {Vec2}
    fn collide_after(&self, other: &Self, dv: Vec2) -> Option<f32> {
        let t = match (dv.x() == 0.0, dv.y() == 0.0) {
            (true, true) => return if self == other { Some(0.0) } else { None },
            (true, false) => (other.x() - self.x()) / dv.x(),
            (false, true) => (other.y() - self.y()) / dv.y(),
            (false, false) => {
                let t = (*other - *self) / dv;
                if f32::abs(t.x() - t.y()) <= EPSILON { t.x() }
                else { return None; }
            }
        };
        if t >= 0.0 && t <= 1.0 {
            Some(1.0 - t)
        } else {
            None
        }
    }
);

impl_collide!(for {AABox, Vec2}
    fn collide_after(&self, other: &Vec2, dv: Vec2) -> Option<f32> {
        match left_under(self.pos, *other) && left_under(*other, self.pos + self.size) {
            false => None,
            true => {
                let (start, end) = (self.pos, self.pos + self.size);
                let p = *other - dv;
                if dv.x() >= 0 {
                    let tx = (end.x() - *other.x()) / dv.x()
                }
            }
        }
    }
);
