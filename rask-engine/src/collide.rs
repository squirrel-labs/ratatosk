//! The collide module provides the Collide trait for objects that can collide along with several
//! implementations for various types.

use core::ops::Range;

use crate::boxes::{AABox, RBox};
use crate::math::Vec2;
use spine::skeleton::SRT;

// For information on the SAT, see: http://www.dyn4j.org/2010/01/sat/.

/// A trait for objects that can collide with other objects.
pub trait Collide<Rhs: ?Sized = Self> {
    fn collides(&self, other: &Rhs) -> bool;
}

/// A trait for common objects to be collidable with other common objects.
pub trait Collidable: Collide<RBox> + Collide<SRT> + Collide<AABox> + Collide<Vec2> {}

fn left_under(v1: Vec2, v2: Vec2) -> bool {
    v1.x() < v2.x() && v1.y() < v2.y()
}

#[derive(Debug)]
struct Projection {
    min: f32,
    max: f32,
}

impl Collide for Projection {
    fn collides(&self, other: &Self) -> bool {
        self.max >= other.min && self.min <= other.max
    }
}

fn project(rbox: &RBox, axis: Vec2) -> Projection {
    // the vertices of rbox without rbox.pos
    let vertices = [
        rbox.pos + rbox.v1,
        rbox.pos + rbox.v2,
        rbox.pos + rbox.v1 + rbox.v2,
    ];
    // project each vertex onto axis
    let p = axis.dot(rbox.pos);
    let mut proj = Projection { min: p, max: p };
    for &vertex in vertices.iter() {
        let p = axis.dot(vertex);
        if p < proj.min {
            proj.min = p;
        } else if p > proj.max {
            proj.max = p;
        }
    }
    proj
}

/// Calculate the bound in a line segment that collides an AABox projected onto an axis.
/// `bound` is a tuple of the start and ending point of the AABB.
/// `pos` is a component of the position vector of the line segment.
/// `direction` is a component of the direction vector of the line segment.
fn calculate_aabox_rbox_component_bounds(
    bound: Range<f32>,
    pos: f32,
    direction: f32,
) -> (f32, f32) {
    if direction == 0.0 {
        return (0.0, 1.0);
    }
    // get bounds of s by transforming "g(s) = pos + s * direction"
    // and applying the inequality g(s) >= bound.start and g(s) <= bound.end
    let (s1, s2) = (
        (bound.start - pos) / direction,
        (bound.end - pos) / direction,
    );
    // if direction is negative, you have to switch the values
    if direction > 0.0 {
        (s1, s2)
    } else {
        (s2, s1)
    }
}

/// Test for collision between an AABox and an edge of a rbox
fn collide_aabox_rbox_segment(
    xbound: Range<f32>,
    ybound: Range<f32>,
    pos: Vec2,
    direction: Vec2,
) -> bool {
    let sbound1 = calculate_aabox_rbox_component_bounds(xbound, pos.x(), direction.x());
    if sbound1.0 > sbound1.1 {
        return false;
    }
    let sbound2 = calculate_aabox_rbox_component_bounds(ybound, pos.y(), direction.y());
    if sbound2.0 > sbound2.1 {
        return false;
    }
    let (sbound1, sbound2) = (sbound1.0..sbound1.1, sbound2.0..sbound2.1);

    sbound1.end >= sbound2.start
        && sbound1.start <= sbound2.end
        && sbound1.end >= 0.0
        && sbound2.end >= 0.0
        && sbound1.start <= 1.0
        && sbound2.start <= 1.0
}

impl Collide for Vec2 {
    fn collides(&self, other: &Self) -> bool {
        self == other
    }
}

impl Collide<AABox> for Vec2 {
    fn collides(&self, other: &AABox) -> bool {
        other.collides(self)
    }
}

impl Collide<RBox> for Vec2 {
    fn collides(&self, other: &RBox) -> bool {
        other.collides(self)
    }
}

impl Collide<SRT> for Vec2 {
    fn collides(&self, other: &SRT) -> bool {
        other.collides(self)
    }
}

impl Collide<Vec2> for AABox {
    fn collides(&self, other: &Vec2) -> bool {
        left_under(self.pos, *other) && left_under(*other, self.pos + self.size)
    }
}

impl Collide<RBox> for AABox {
    fn collides(&self, other: &RBox) -> bool {
        other.collides(self)
    }
}

impl Collide<SRT> for AABox {
    fn collides(&self, other: &SRT) -> bool {
        other.collides(self)
    }
}

impl Collide for AABox {
    fn collides(&self, other: &Self) -> bool {
        left_under(self.pos, other.pos + other.size) && left_under(other.pos, self.pos + self.size)
    }
}

impl Collide<Vec2> for RBox {
    fn collides(&self, other: &Vec2) -> bool {
        let v1_proj = project(self, self.v1);
        let p1 = other.dot(self.v1);
        let v2_proj = project(self, self.v2);
        let p2 = other.dot(self.v2);
        v1_proj.min <= p1 && v1_proj.max >= p1 && v2_proj.min <= p2 && v2_proj.max >= p2
    }
}

impl Collide<Vec2> for SRT {
    fn collides(&self, other: &Vec2) -> bool {
        let rbox: RBox = self.into();
        rbox.collides(other)
    }
}

impl Collide<AABox> for SRT {
    fn collides(&self, other: &AABox) -> bool {
        let rbox: RBox = self.into();
        rbox.collides(other)
    }
}

impl Collide<RBox> for SRT {
    fn collides(&self, other: &RBox) -> bool {
        let rbox: RBox = self.into();
        rbox.collides(other)
    }
}

impl Collide for SRT {
    fn collides(&self, other: &Self) -> bool {
        let (rbox1, rbox2): (RBox, RBox) = (self.into(), other.into());
        rbox1.collides(&rbox2)
    }
}

impl Collide<AABox> for RBox {
    fn collides(&self, other: &AABox) -> bool {
        let xbound = other.pos.x()..other.pos.x() + other.size.x();
        let ybound = other.pos.y()..other.pos.y() + other.size.y();
        collide_aabox_rbox_segment(xbound.clone(), ybound.clone(), self.pos, self.v1)
            || collide_aabox_rbox_segment(xbound.clone(), ybound.clone(), self.pos, self.v2)
            || collide_aabox_rbox_segment(
                xbound.clone(),
                ybound.clone(),
                self.pos + self.v1,
                self.v2,
            )
            || collide_aabox_rbox_segment(xbound, ybound, self.pos + self.v2, self.v1)
    }
}

impl Collide<SRT> for RBox {
    fn collides(&self, other: &SRT) -> bool {
        let rbox: RBox = other.into();
        rbox.collides(self)
    }
}

impl Collide for RBox {
    fn collides(&self, other: &Self) -> bool {
        // using the SAT
        // TODO: optimization: remove duplicate axes
        let axes = [self.v1, self.v2, other.v1, other.v2];
        axes.iter()
            .all(|axis| project(self, *axis).collides(&project(other, *axis)))
    }
}

impl<S, T: Collide<S>> Collide<S> for [T] {
    fn collides(&self, other: &S) -> bool {
        self.iter().any(|x| x.collides(other))
    }
}

impl<S, T: Collide<S>> Collide<[T]> for S {
    fn collides(&self, other: &[T]) -> bool {
        other.collides(self)
    }
}

impl Collidable for RBox {}
impl Collidable for SRT {}
impl Collidable for AABox {}
impl Collidable for Vec2 {}
