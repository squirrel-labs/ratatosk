//! The collide module provides the Collide trait for objects that can collide along with several
//! implementations for various types.

use crate::boxes::{AABox, RBox};
use crate::math::{Vec2, EPSILON};

// For information on the SAT, see: http://www.dyn4j.org/2010/01/sat/.

/// A trait for objects that can collide with other objects.
pub trait Collide<Rhs = Self> {
    /// calculate the penetration depth (a scalar ratio of `dv`)
    /// of the collision after applying the velocity,
    /// if there was any.
    /// `dv` is the difference between the two velocities
    fn collide_after(&self, other: &Rhs, dv: Vec2) -> Option<f32>;
}

pub enum Collidable {
    Point(Vec2),
    AABox(AABox),
    RBox(RBox),
}

impl Collide for Collidable {
    fn collide_after(&self, other: &Self, dv: Vec2) -> Option<f32> {
        match (self, other) {
            (&Self::Point(ref a), &Self::Point(ref b)) => a.collide_after(b, dv),
            (&Self::Point(ref a), &Self::AABox(ref b)) => a.collide_after(b, dv),
            //(Self::Point(a), Self::RBox(b)) => a.collide_after(b, dv),
            (&Self::AABox(ref a), &Self::Point(ref b)) => a.collide_after(b, dv),
            (&Self::AABox(ref a), &Self::AABox(ref b)) => a.collide_after(b, dv),
            //(Self::AABox(a), Self::RBox(b)) => a.collide_after(b, dv),
            //(Self::RBox(a), Self::Point(b)) => a.collide_after(b, dv),
            //(Self::RBox(a), Self::AABox(b)) => a.collide_after(b, dv),
            //(Self::RBox(a), Self::RBox(b)) => a.collide_after(b, dv),
            // TODO: remove following:
            _ => None,
        }
    }
}

fn left_under(v1: Vec2, v2: Vec2) -> bool {
    v1.x() < v2.x() && v1.y() < v2.y()
}

fn in_range(t: f32) -> Option<f32> {
    if (0.0..=1.0).contains(&t) {
        Some(1.0 - t)
    } else {
        None
    }
}

fn project(rbox: &RBox, axis: Vec2) -> (f32, f32) {
    // the vertices of rbox without rbox.pos
    let vertices = [
        rbox.pos + rbox.v1,
        rbox.pos + rbox.v2,
        rbox.pos + rbox.v1 + rbox.v2,
    ];
    // project each vertex onto axis
    let p = axis.dot(rbox.pos);
    let mut proj = (p, p);
    for &vertex in vertices.iter() {
        let p = axis.dot(vertex);
        if p < proj.0 {
            proj.0 = p;
        } else if p > proj.0 {
            proj.1 = p;
        }
    }
    proj
}

macro_rules! impl_collide {
    (for {$A:ident} $item:item) => {
        impl Collide for $A {
            $item
        }
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

impl_collide!(for {Vec2, AABox}
    fn collide_after(&self, other: &AABox, dv: Vec2) -> Option<f32> {
        let left = || ((self.x() + dv.x() - other.pos.x()) / dv.x());
        let bottom = || ((self.y() + dv.y() - other.pos.y()) / dv.y());
        let right = || ((self.x() + dv.x() - other.pos.x() - other.size.x()) / dv.x());
        let top = || ((self.y() + dv.y() - other.pos.y() - other.size.y()) / dv.y());
        if left_under(other.pos, *self) && left_under(*self, other.pos + other.size) {
            Some(if dv.x() > 0.0 {
                     if dv.y() > 0.0 {
                         bottom().min(left())
                     } else {
                         top().min(left())
                     }
                 } else if dv.y() > 0.0 {
                     bottom().min(right())
                 } else {
                     top().min(right())
                 })
        } else { None }
    }
);

impl_collide!(for {AABox}
    fn collide_after(&self, other: &Self, dv: Vec2) -> Option<f32> {
        let left = || (self.pos.x() + dv.x() - other.pos.x()) / dv.x();
        let bottom = || (self.pos.y() + dv.y() - other.pos.y()) / dv.y();
        let right = || (self.pos.x() + self.size.x() + dv.x() - other.pos.x()) / dv.x();
        let top = || (self.pos.y() + self.size.y() + dv.y() - other.pos.y()) / dv.y();
        if left_under(self.pos, other.pos + other.size)
                && left_under(other.pos, self.pos + self.size) {
            Some(if dv.x() > 0.0 {
                     if dv.y() > 0.0 {
                         bottom().min(left())
                     } else {
                         top().min(left())
                     }
                 } else if dv.y() > 0.0 {
                     bottom().min(right())
                 } else {
                     top().min(right())
                 })
        } else {
            None
        }
    }
);

impl_collide!(for {Vec2, RBox}
    fn collide_after(&self, other: &RBox, dv: Vec2) -> Option<f32> {
        let p = *self + dv;
        let (amin, amax) = project(other, other.v1);
        let a = p.dot(other.v1);
        let (bmin, bmax) = project(other, other.v2);
        let b = p.dot(other.v2);
        if (amin..=amax).contains(&a) && (bmin..=bmax).contains(&b) {
            let f = |v, a, min, max| {
                let t = self.dot(v);
                if t < min {
                    Some((a - min) / (a - t))
                } else if t > max {
                    Some((a - max) / (a - t))
                } else {
                    None
                }
            };
            Some(match (f(other.v1, a, amin, amax), f(other.v2, b, bmin, bmax)) {
                (Some(a), Some(b)) => a.min(b),
                (Some(a), None) | (None, Some(a)) => a,
                _ => 1.0
            })
        } else {
            None
        }
    }
);

impl_collide!(for {RBox, AABox}
    fn collide_after(&self, other: &AABox, dv: Vec2) -> Option<f32> {
        let rbox = self.as_normal_form();
        let [left, right, top, bottom] = [
            rbox.pos + rbox.v1,
            rbox.pos + rbox.v2,
            rbox.pos + rbox.v1 + rbox.v2,
            rbox.pos,
        ];

        // corner of rbox moves into endge of aabox:
        // ----------------------------------------

        let (x, y) = (dv.x(), dv.y());

        let edges = if dv.x() >= 0.0 {
            if dv.y() >= 0.0 {
                [(right.x(), other.pos.x(), x), (top.y(), other.pos.y(), y)]
            } else {
                [
                    (right.x(), other.pos.x(), x),
                    (bottom.y(), other.pos.y() + other.size.y(), y),
                ]
            }
        } else if dv.y() >= 0.0 {
            [
                (left.x(), other.pos.x() + other.size.x(), x),
                (top.y(), other.pos.y(), y),
            ]
        } else {
            [
                (left.x(), other.pos.x() + other.size.x(), x),
                (bottom.y(), other.pos.y() + other.size.y(), y),
            ]
        };
        let mut ts = edges.iter().filter_map(|(start, end, delta)| {
            let t = 1.0 + (start - end) / delta;
            if (0.0..=1.0).contains(&t) {
                Some(t)
            } else {
                None
            }
        });
        let mut t_min = ts
            .next()
            .map(|t1| ts.next().filter(|t2| t2 < &t1).unwrap_or(t1));

        // edge of rbox moves into corner of aabox:
        // ----------------------------------------

        let vdotvn = (dv.dot(rbox.v1), dv.dot(rbox.v2));
        for &(start, end, vn, dot) in &[
            (
                right,
                other.pos + Vec2::new(0.0, other.size.y()),
                rbox.v1,
                vdotvn.0,
            ),
            (right, other.pos, rbox.v2, vdotvn.1),
            (left, other.pos + other.size, rbox.v2, vdotvn.1),
            (
                left,
                other.pos + Vec2::new(other.size.x(), 0.0),
                rbox.v1,
                vdotvn.0,
            ),
        ] {
            let t = (start - end).dot(vn) / dot;
            if (0.0..=1.0).contains(&t) && t < t_min.unwrap_or(1.0) {
                t_min = Some(t)
            }
        }

        t_min
    }
);

fn collide_rbox_rbox_helper(b1: &RBox, b2: &RBox, dv: Vec2) -> Option<f32> {
    let [left1, right1, top1, bottom1] = [
        b1.pos + b1.v1,
        b1.pos + b1.v2,
        b1.pos + b1.v1 + b1.v2,
        b1.pos,
    ];
    let [left2, right2, bottom2] = [b2.pos + b2.v1, b2.pos + b2.v2, b2.pos];
    let dotv1 = dv.dot(b2.v1);
    let dotv2 = dv.dot(b2.v2);
    let lines = if dotv1 < 0.0 {
        if dotv2 < 0.0 {
            [(left2, b2.v2), (right2, b2.v1)]
        } else {
            [(bottom2, b2.v1), (left2, b2.v2)]
        }
    } else if dotv2 < 0.0 {
        [(bottom2, b2.v2), (right2, b2.v1)]
    } else {
        [(bottom2, b2.v1), (bottom2, b2.v2)]
    };
    let mut t = None;
    for p in &[left1, right1, top1, bottom1] {
        for (start, v) in &lines {
            let t_ = (v.y() * (start.x() - p.x()) + v.x() * (p.y() - start.y()))
                / (dv.dot(Vec2::new(v.y(), v.x())));
            if (0.0..=1.0).contains(&t_) && t.map(|t| t_ > t).unwrap_or(true) {
                t = Some(t_)
            }
        }
    }
    t
}

impl_collide!(for {RBox}
    fn collide_after(&self, other: &RBox, dv: Vec2) -> Option<f32> {
        let b1 = self.as_normal_form();
        let b2 = other.as_normal_form();
        let t = collide_rbox_rbox_helper(&b1, &b2, dv);
        collide_rbox_rbox_helper(&b2, &b1, dv).and_then(|t1| t.filter(|t2| t2 > &t1)).or(t)
    }
);
