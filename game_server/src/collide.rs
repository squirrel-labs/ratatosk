use crate::maths::{Vec2, AABox, RBox};

pub trait Collide<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

impl Collide<Vec2> for Vec2 {
    fn collides(&self, other: &Self) -> bool {
        self == other
    }
}

impl Collide<Vec2> for AABox {
    fn collides(&self, other: &Vec2) -> bool {
        self.pos < *other && other < &(self.pos + self.size) 
    }
}

impl Collide<AABox> for AABox {
    fn collides(&self, other: &Self) -> bool {
        self.pos.x < other.pos.x + other.size.x && other.pos.x < self.pos.x + self.size.x
        && self.pos.y < other.pos.y + other.size.y && other.pos.y < self.pos.y + self.size.y
    }
}

impl Collide<Vec2> for RBox {
    fn collides(&self, other: &Vec2) -> bool {
        let v1_diff = *other + self.v1 * (-self.v1.scalar(&(*other - self.pos)) / self.v1.distance2());
        let v2_diff = *other + self.v2 * (-self.v2.scalar(&(*other - self.pos)) / self.v2.distance2());

        let v1_dist = ((v1_diff - self.pos) / self.v2).x;
        let v2_dist = ((v2_diff - self.pos) / self.v1).x;
        0.0 <= v1_dist && v2_dist <= 1.0
        && 0.0 <= v2_dist && v2_dist <= 1.0
        //v1_diff < self.pos + self.v2 && self.pos < v1_diff
        //&& v2_diff < self.pos + self.v1 && self.pos < v2_diff
    }
}

impl Collide<AABox> for RBox {
    fn collides(&self, other: &AABox) -> bool {
        let other_size = other.pos + other.size;

        // project points onto a orthogonal line
        let v1_diff = other.pos + self.v1 * (-self.v1.scalar(&(other.pos - self.pos)) / self.v1.distance2());
        let v2_diff = other.pos + self.v2 * (-self.v2.scalar(&other.pos) / self.v2.distance2());
        let v1_diff_size = other_size + self.v1 * (-self.v1.scalar(&(other_size - self.pos)) / self.v1.distance2());
        let v2_diff_size = other_size + self.v2 * (-self.v2.scalar(&(other_size - self.pos)) / self.v2.distance2());

        // calculate the distance
        let v1_dist = ((v1_diff - self.pos) / self.v2);
        let v2_dist = ((v2_diff - self.pos) / self.v1);
        let v1_dist_size = ((v1_diff_size - self.pos) / self.v2);
        let v2_dist_size = ((v2_diff_size - self.pos) / self.v1);

        let v1_dist = if v1_dist.x.is_finite() {v1_dist.x} else {v1_dist.y};
        let v2_dist = if v2_dist.x.is_finite() {v2_dist.x} else {v2_dist.y};
        let v1_dist_size = if v1_dist_size.x.is_finite() {v1_dist_size.x} else {v1_dist_size.y};
        let v2_dist_size = if v2_dist_size.x.is_finite() {v2_dist_size.x} else {v2_dist_size.y};

        let minx = f32::min(self.pos.x, f32::min((self.pos + self.v1).x, (self.pos + self.v2).x));
        let maxx = f32::max(self.pos.x, f32::max((self.pos + self.v1).x, (self.pos + self.v2).x));
        let miny = f32::min(self.pos.y, f32::min((self.pos + self.v1).y, (self.pos + self.v2).y));
        let maxy = f32::max(self.pos.y, f32::max((self.pos + self.v1).y, (self.pos + self.v2).y));

        0.0 <= v1_dist_size && v1_dist <= 1.0
        && 0.0 <= v2_dist_size && v2_dist <= 1.0
        && other.pos.x < maxx && minx < other.pos.x + other.size.x
        && other.pos.y < maxy && miny < other.pos.y + other.size.y
    }
}

impl<S, T: Collide<S>> Collide<S> for Vec<T> {
    fn collides(&self, other: &S) -> bool {
        self.iter().any(|x| x.collides(other))
    }
}

#[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_collide_dot_dot() {
             let a = Vec2{x: 1.0, y: 7.5};
             assert!(a.collides(&a));
        }
    
        #[test]
        fn test_not_collide_dot_dot() {
             let a = Vec2{x: 1.0, y: 7.5};
             let b = Vec2{x: 5.0, y: 7.5};
             assert!(!a.collides(&b));
        }

        #[test]
        fn test_collide_aabox_dot() {
             let a = Vec2{x: 1.0, y: 2.5};
             let b = Vec2{x: 3.0, y: 7.5};
             let c = Vec2{x: 1.5, y: 5.0};
             let aa_box = AABox{pos: a, size: b};

             assert!(aa_box.collides(&c));
        }
    
        #[test]
        fn test_not_collide_aabox_dot() {
             let a = Vec2{x: 1.0, y: 7.5};
             let b = Vec2{x: 3.0, y: 2.5};
             let c = Vec2{x: 0.5, y: 5.0};
             let aa_box = AABox{pos: a, size: b};

             assert!(!(aa_box.collides(&c)));
        }
        
        #[test]
        fn test_collide_aabox_aabox_intersecting() {
             let a = Vec2{x: 1.0, y: 2.5};
             let b = Vec2{x: 3.0, y: 2.5};
             let aa_box = AABox{pos: a, size: b};
             let a = Vec2{x: 2.0, y: 3.5};
             let b = Vec2{x: 3.0, y: 7.5};
             let bb_box = AABox{pos: a, size: b};

             assert!(aa_box.collides(&bb_box));
        }

        #[test]
        fn test_collide_aabox_aabox_crossed() {
             let a = Vec2{x: 2.0, y: 0.5};
             let b = Vec2{x: 1.0, y: 7.5};
             let aa_box = AABox{pos: a, size: b};
             let a = Vec2{x: 1.0, y: 3.5};
             let b = Vec2{x: 5.0, y: 4.5};
             let bb_box = AABox{pos: a, size: b};

             assert!(aa_box.collides(&bb_box));
        }
    
        #[test]
        fn test_not_collide_aabox_aabox() {
             let a = Vec2{x: 1.0, y: 1.0};
             let b = Vec2{x: 1.0, y: 1.0};
             let aa_box = AABox{pos: a, size: b};
             let a = Vec2{x: 3.0, y: 3.5};
             let b = Vec2{x: 3.0, y: 7.5};
             let bb_box = AABox{pos: a, size: b};

             assert!(!(aa_box.collides(&bb_box)));
        }

        #[test]
        fn test_collide_Rbox_dot() {
             let a = Vec2{x: 1.0, y: 1.0};
             let b = Vec2{x: 1.0, y: 1.0};
             let c = Vec2{x: 1.0, y: -1.0};
             let aa_box = RBox{pos: a, v1: b, v2: c};

             let c = Vec2{x: 1.6, y: 0.6};

             assert!(aa_box.collides(&c));
        }
    
        #[test]
        fn test_not_collide_Rbox_dot() {
             let a = Vec2{x: 1.0, y: 1.0};
             let b = Vec2{x: 1.0, y: 1.0};
             let c = Vec2{x: 1.0, y: -1.0};
             let aa_box = RBox{pos: a, v1: b, v2: c};

             let c = Vec2{x: 1.4, y: 0.4};

             assert!(!(aa_box.collides(&c)));
        }

        #[test]
        fn test_collide_rbox_rbox_intersecting() {
             let a = Vec2{x: 1.0, y: 2.5};
             let b = Vec2{x: 0.0, y: 2.5};
             let c = Vec2{x: 3.0, y: 0.5};
             let aa_box = RBox{pos: a, v1: b, v2: c};
             let a = Vec2{x: 2.0, y: 3.5};
             let b = Vec2{x: 3.0, y: 7.5};
             let bb_box = AABox{pos: a, size: b};

             assert!(aa_box.collides(&bb_box));
        }

        #[test]
        fn test_collide_rbox_rbox_crossed() {
             let a = Vec2{x: 2.0, y: 0.5};
             let b = Vec2{x: 1.0, y: 0.0};
             let c = Vec2{x: 0.0, y: 7.5};
             let aa_box = RBox{pos: a, v1: b, v2: c};
             let a = Vec2{x: 1.0, y: 3.5};
             let b = Vec2{x: 5.0, y: 4.5};
             let bb_box = AABox{pos: a, size: b};

             assert!(aa_box.collides(&bb_box));
        }
    
        #[test]
        fn test_not_collide_rbox_rbox() {
             let a = Vec2{x: 1.0, y: 1.0};
             let b = Vec2{x: 0.0, y: 1.0};
             let c = Vec2{x: 1.0, y: 0.0};
             let aa_box = RBox{pos: a, v1: b, v2: c};
             let a = Vec2{x: 3.0, y: 3.5};
             let b = Vec2{x: 3.0, y: 7.5};
             let bb_box = AABox{pos: a, size: b};

             assert!(!(aa_box.collides(&bb_box)));
        }

}
