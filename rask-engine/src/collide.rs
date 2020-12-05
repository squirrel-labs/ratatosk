//! The collide module provides the Collide trait for objects that can collide along with several
//! implementations for various types.

use crate::boxes::{AABox, RBox};
use crate::math::{Vec2, EPSILON};
use spine::skeleton::srt::SRT;

// For information on the SAT, see: http://www.dyn4j.org/2010/01/sat/.

/// A trait for objects that can collide with other objects.
pub trait Collide<Rhs = Self> {
    /// calculate the penetration depth (a scalar ratio of `dv`)
    /// of the collision after applying the velocity,
    /// if there was any.
    /// `dv` is the difference between the two velocities
    fn collide_after(&self, other: &Rhs, dv: Vec2) -> Option<f32>;
}

/// A trait for common objects to be collidable with other common objects.
pub trait Collidable: Collide<RBox> + Collide<SRT> + Collide<AABox> + Collide<Vec2> {}

fn left_under(v1: Vec2, v2: Vec2) -> bool {
    v1.x() < v2.x() && v1.y() < v2.y()
}

fn in_range(t: f32) -> Option<f32> {
    if t >= 0.0 && t <= 1.0 {
        Some(1.0 - t)
    } else {
        None
    }
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
        in_range(match (dv.x() == 0.0, dv.y() == 0.0) {
            (true, true) => return if self == other { Some(0.0) } else { None },
            (true, false) => (other.x() - self.x()) / dv.x(),
            (false, true) => (other.y() - self.y()) / dv.y(),
            (false, false) => {
                let t = (*other - *self) / dv;
                if f32::abs(t.x() - t.y()) <= EPSILON { t.x() }
                else { return None; }
            }
        })
    }
);

macro_rules! lbrt {
    ($other:expr, $dv:expr) => {{
        let left = |slf: &Vec2| ((slf.x() + $dv.x() - $other.pos.x()) / $dv.x());
        let bottom = |slf: &Vec2| ((slf.y() + $dv.y() - $other.pos.y()) / $dv.y());
        let right = |slf: &Vec2| ((slf.x() + $dv.x() - $other.pos.x() - $other.size.x()) / $dv.x());
        let top = |slf: &Vec2| ((slf.y() + $dv.y() - $other.pos.y() - $other.size.y()) / $dv.y());
        (left, bottom, right, top)
    }};
}

impl_collide!(for {Vec2, AABox}
    fn collide_after(&self, other: &AABox, dv: Vec2) -> Option<f32> {
        let (left, bottom, right, top) = lbrt!(other, dv);
        if left_under(other.pos, *self) && left_under(*self, other.pos + other.size) {
            Some(if dv.x() > 0.0 {
                     if dv.y() > 0.0 {
                         bottom(self).min(left(self))
                     } else {
                         top(self).min(left(self))
                     }
                 } else if dv.y() > 0.0 {
                     bottom(self).min(right(self))
                 } else {
                     top(self).min(right(self))
                 })
        } else { None }
    }
);

impl_collide!(for {AABox}
    fn collide_after(&self, other: &Self, dv: Vec2) -> Option<f32> {
        let (left, bottom, right, top) = lbrt!(other, dv);
        macro_rules! comb { ($f:expr, $g:expr, $slf:expr) => {{
            let z = $slf;
            Some($f(z).min($g(z)))
        }}};
        if left_under(self.pos, other.pos + other.size)
                && left_under(other.pos, self.pos + self.size) {
            if dv.x() > 0.0 {
                if dv.y() > 0.0 {
                    comb!(bottom, left, &(self.pos + self.size))
                } else {
                    comb!(top, left, &(Vec2::new(self.pos.x() + self.size.x(), self.pos.y())))
                }
            } else if dv.y() > 0.0 {
                comb!(bottom, right, &(Vec2::new(self.pos.x(), self.pos.y() + self.size.y())))
            } else {
                comb!(top, right, &self.pos)
            }
        } else {
            None
        }
    }
);
