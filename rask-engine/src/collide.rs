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

#[derive(Debug, Clone)]
pub enum Collidable {
    Point(Vec2),
    AABox(AABox),
    RBox(RBox),
}

impl Collide for Collidable {
    fn collide_after(&self, other: &Self, dv: Vec2) -> Option<f32> {
        match (self, other) {
            (Self::Point(a), Self::Point(b)) => a.collide_after(b, dv),
            (Self::Point(a), Self::AABox(b)) => a.collide_after(b, dv),
            (Self::Point(a), Self::RBox(b)) => a.collide_after(b, dv),
            (Self::AABox(a), Self::Point(b)) => a.collide_after(b, dv),
            (Self::AABox(a), Self::AABox(b)) => a.collide_after(b, dv),
            (Self::AABox(a), Self::RBox(b)) => a.collide_after(b, dv),
            (Self::RBox(a), Self::Point(b)) => a.collide_after(b, dv),
            (Self::RBox(a), Self::AABox(b)) => a.collide_after(b, dv),
            (Self::RBox(a), Self::RBox(b)) => a.collide_after(b, dv),
        }
    }
}

/// calculate the axis aligned bounding box for a set of collidables
pub fn calculate_aabb<'a, I: Iterator<Item = &'a Collidable>>(vals: I) -> AABox {
    let (mut minx, mut maxx, mut miny, mut maxy) = (0.0, 0.0, 0.0, 0.0);
    let mut first_iter = true;
    let mut f = |points: &[Vec2]| {
        for point in points {
            if first_iter {
                minx = point.x();
                maxx = point.x();
                miny = point.y();
                maxy = point.y();
                first_iter = false;
            } else {
                minx = f32::min(minx, point.x());
                maxx = f32::max(maxx, point.x());
                miny = f32::min(miny, point.y());
                maxy = f32::max(maxy, point.y());
            }
        }
    };
    for val in vals {
        match val {
            Collidable::Point(v) => f(&[*v]),
            Collidable::AABox(AABox { pos, size }) => f(&[*pos, *pos + *size]),
            Collidable::RBox(RBox { pos, v1, v2 }) => {
                f(&[*pos, *pos + *v1, *pos + *v2, *pos + *v1 + *v2])
            }
        };
    }
    AABox {
        pos: Vec2::new(minx, miny),
        size: Vec2::new(maxx - minx, maxy - miny),
    }
}

// Given a fraction of the dv vectors length,
// return the amount to set the objects back
// in case of a collision
fn in_range(t: f32) -> Option<f32> {
    if (0.0..=1.0).contains(&t) {
        Some(1.0 - t)
    } else {
        None
    }
}

macro_rules! impl_collide {
    (for {$A:ident} $item:item) => { impl_collide!(for {$A/$A} $item); };
    (for {$A:ident/$C:ident} $item:item) => {
        impl Collide for $A {
            $item
        }

        impl From<$A> for Collidable {
            fn from(other: $A) -> Self {
                Self::$C(other)
            }
        }
    };
    (for {$A:ident, $B:ident} $item:item) => {
        impl Collide<$B> for $A {
            $item
        }
        impl Collide<$A> for $B {
            fn collide_after(&self, other: &$A, dv: Vec2) -> Option<f32> {
                other.collide_after(self, -dv)
            }
        }
    };
}

impl_collide!(for {Vec2/Point}
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
        // test collision between a line starting at a with the vector v
        // and another line starting at p with the vector (0, w).
        let line_col = |a: Vec2, v: Vec2, p: Vec2, w| {
            let t = (p.x() - a.x()) / v.x();
            if (0.0..=w).contains(&(a.y() - p.y() + v.y() * t)) {
                in_range(t)
            } else{ None }
        };
        let rev = |v: Vec2| Vec2::new(v.y(), v.x());
        let left = || line_col(*self, dv, other.pos, other.size.y());
        let right = || line_col(*self, dv, other.pos + Vec2::new(other.size.x(), 0.0), other.size.y());
        let bottom = || line_col(rev(*self), rev(dv), rev(other.pos), other.size.x());
        let top = || line_col(rev(*self), rev(dv), rev(other.pos + Vec2::new(0.0, other.size.y())), other.size.x());
        (if dv.x() >= 0.0 { left() } else { right() })
            .or_else(|| if dv.y() >= 0.0 { bottom() } else { top() })
    }
);

impl_collide!(for {AABox}
    fn collide_after(&self, other: &Self, dv: Vec2) -> Option<f32> {
        type F = fn(&f32, &f32) -> bool;
        let lines_col = |a: Vec2, b: Vec2, w: f32, u: f32, v: Vec2, f1: F, f2: F| {
            let z = v.y() * (a.x() - b.x()) + v.x() * (b.y() - a.y());
            let (s1, s2) = (u * v.x() + z, w * v.x());
            if f1(&s1, &0.0) && f2(&z, &s2) {
                let t = (b.x() - a.x()) / v.x();
                in_range(t)
            } else {
                None
            }
        };
        let lines_col_p = |a: Vec2, b: Vec2, w: f32, u: f32, v: Vec2| {
            lines_col(a, b, w, u, v, PartialOrd::ge, PartialOrd::le)
        };
        let lines_col_n = |a: Vec2, b: Vec2, w: f32, u: f32, v: Vec2| {
            lines_col(a, b, w, u, v, PartialOrd::le, PartialOrd::ge)
        };
        let rev = |v: Vec2| Vec2::new(v.y(), v.x());
        let left = || {
            lines_col_p(
                self.pos + Vec2::new(self.size.x(), 0.0),
                other.pos,
                self.size.y(),
                other.size.y(),
                dv,
            )
        };
        let right = || {
            lines_col_n(
                self.pos,
                other.pos + Vec2::new(other.size.x(), 0.0),
                self.size.y(),
                other.size.y(),
                dv,
            )
        };
        let bottom = || {
            lines_col_p(
                rev(self.pos + Vec2::new(0.0, self.size.y())),
                rev(other.pos),
                self.size.x(),
                other.size.x(),
                rev(dv),
            )
        };
        let top = || {
            lines_col_n(
                rev(self.pos),
                rev(other.pos + Vec2::new(0.0, other.size.y())),
                self.size.x(),
                other.size.x(),
                rev(dv),
            )
        };
        (if dv.x() >= 0.0 { left() } else { right() })
            .or_else(|| if dv.y() >= 0.0 { bottom() } else { top() })
    }
);

fn line_intersect(a: Vec2, v: Vec2, b: Vec2, w: Vec2) -> Option<f32> {
    let t = 1.0
        - (w.x() * (b.y() - a.y() - v.y()) + w.y() * (v.x() + a.x() - b.x()))
            / (v.x() * w.y() - v.y() * w.x());
    let k = a.x() - b.x() + v.x() * t;
    if (0.0..=w.x()).contains(&k) || (w.x()..=0.0).contains(&k) {
        in_range(t)
    } else {
        None
    }
}

impl_collide!(for {Vec2, RBox}
    fn collide_after(&self, other: &RBox, dv: Vec2) -> Option<f32> {
        let rbox = other.as_normal_form();
        let col_lines = |b, w| line_intersect(*self, dv, b, w);

        let left = || col_lines(rbox.pos, rbox.v1);
        let right = || col_lines(rbox.pos + rbox.v2, rbox.v1);
        let top = || col_lines(rbox.pos + rbox.v1, rbox.v2);
        let bottom = || col_lines(rbox.pos, rbox.v2);
        (if dv.dot(rbox.v2) >= 0.0 { left() } else { right() })
            .or_else(|| if dv.dot(rbox.v1) >= 0.0 { bottom() } else { top() })
    }
);

impl_collide!(for {RBox, AABox}
    fn collide_after(&self, other: &AABox, dv: Vec2) -> Option<f32> {
        let rbox = self.as_normal_form();
        let (w, h) = (
            other.pos + Vec2::new(other.size.x(), 0.0),
            other.pos + Vec2::new(0.0, other.size.y()),
        );

        if rbox.v1.x() == 0.0 {
            return AABox {
                pos: rbox.pos,
                size: Vec2::new(rbox.v2.x(), rbox.v1.y()),
            }
            .collide_after(other, dv);
        }

        // corner of rbox moves into edge of aabox:
        // ----------------------------------------

        let line_col = |a: Vec2, v: Vec2, p: Vec2, w| {
            let t = (p.x() - a.x()) / v.x();
            if (0.0..=1.0).contains(&t) && (0.0..=w).contains(&(a.y() - p.y() + v.y() * t)) {
                Some(1.0 - t)
            } else {
                None
            }
        };
        let rev = |v: Vec2| Vec2::new(v.y(), v.x());
        let left = || line_col(rbox.pos + rbox.v2, dv, other.pos, other.size.y());
        let right = || line_col(rbox.pos + rbox.v1, dv, w, other.size.y());
        let bottom = || {
            line_col(
                rev(rbox.pos + rbox.v1 + rbox.v2),
                rev(dv),
                rev(other.pos),
                other.size.x(),
            )
        };
        let top = || line_col(rev(rbox.pos), rev(dv), rev(h), other.size.x());
        let mut t = (if dv.x() >= 0.0 { left() } else { right() })
            .or_else(|| if dv.y() >= 0.0 { bottom() } else { top() });

        // edge of rbox moves into corner of aabox:
        // ----------------------------------------

        for &(a, b, w) in &[
            (other.pos + other.size, rbox.pos, rbox.v1),
            (h, rbox.pos, rbox.v2),
            (other.pos, rbox.pos + rbox.v2, rbox.v1),
            (w, rbox.pos + rbox.v1, rbox.v2),
        ] {
            let t_ = line_intersect(a, -dv, b, w);
            if let Some(t_) = t_ {
                if t.filter(|t| t > &t_).is_none() {
                    t = Some(t_)
                }
            }
        }

        t
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
    let mut t: Option<f32> = None;
    for &p in &[left1, right1, top1, bottom1] {
        for &(start, v) in &lines {
            println!("{:?} {:?} {:?} {:?}", p, dv, start, v);
            if let Some(t_) = line_intersect(p, dv, start, v) {
                println!(" -> {:?}", t_);
                if !Option::filter(t, |t| t > &t_).is_some() {
                    t = Some(t_)
                }
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
        collide_rbox_rbox_helper(&b2, &b1, -dv).map(|t1| t.map(|t2| f32::max(t1, t2)).unwrap_or(t1)).or(t)
    }
);
