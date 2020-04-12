//! The collide module provides the Collide trait for objects that can collide along with several
//! implementations for various types.

use crate::boxes::{AABox, RBox};
use crate::math::Vec2;

// For information on the SAT, see: http://www.dyn4j.org/2010/01/sat/.

/// A trait for objects that can collide with other objects.
pub trait Collide<Rhs = Self> {
    fn collides(&self, other: &Rhs) -> bool;
}

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

fn project(rbox: &RBox, axis: &Vec2) -> Projection {
    // the vertices of rbox without rbox.pos
    let vertices = [
        rbox.pos + rbox.v1,
        rbox.pos + rbox.v2,
        rbox.pos + rbox.v1 + rbox.v2,
    ];
    // project each vertex onto axis
    vertices.iter().fold(
        {
            let p = axis.dot(rbox.pos);
            Projection { min: p, max: p }
        },
        |Projection { min, max }, vertex| {
            let p = axis.dot(*vertex);
            if p < min {
                Projection { min: p, max }
            } else if p > max {
                Projection { min, max: p }
            } else {
                Projection { min, max }
            }
        },
    )
}

impl Collide for Vec2 {
    fn collides(&self, other: &Self) -> bool {
        self == other
    }
}

impl Collide<Vec2> for AABox {
    fn collides(&self, other: &Vec2) -> bool {
        left_under(self.pos, *other) && left_under(*other, self.pos + self.size)
    }
}

impl Collide for AABox {
    fn collides(&self, other: &Self) -> bool {
        left_under(self.pos, other.pos + other.size) && left_under(other.pos, self.pos + self.size)
    }
}

impl Collide<Vec2> for RBox {
    fn collides(&self, other: &Vec2) -> bool {
        let v1_proj = project(self, &self.v1);
        let p1 = other.dot(self.v1);
        let v2_proj = project(self, &self.v2);
        let p2 = other.dot(self.v2);
        v1_proj.min <= p1 && v1_proj.max >= p1 && v2_proj.min <= p2 && v2_proj.max >= p2
    }
}

impl Collide<Vec2> for spine::skeleton::SRT {
    fn collides(&self, other: &Vec2) -> bool {
        let rbox: RBox = self.into();
        rbox.collides(other)
    }
}

impl Collide<AABox> for spine::skeleton::SRT {
    fn collides(&self, other: &AABox) -> bool {
        let rbox: RBox = self.into();
        rbox.collides(other)
    }
}

impl Collide<AABox> for RBox {
    fn collides(&self, other: &AABox) -> bool {
        let rbox: RBox = (*other).into();
        self.collides(&rbox)
    }
}

impl Collide for RBox {
    fn collides(&self, other: &Self) -> bool {
        // using the SAT
        // TODO: optimization: remove duplicate axes
        let axes = [self.v1, self.v2, other.v1, other.v2];
        axes.iter()
            .all(|axis| project(self, axis).collides(&project(other, axis)))
    }
}

impl<S, T: Collide<S>> Collide<S> for [T] {
    fn collides(&self, other: &S) -> bool {
        self.iter().any(|x| x.collides(other))
    }
}
