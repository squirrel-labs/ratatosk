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
        let v1_diff = *other + self.v1 * (-self.v1.scalar(&other) / self.v1.distance2());
        let v2_diff = *other + self.v2 * (-self.v2.scalar(&other) / self.v2.distance2());

        self.pos < v1_diff && v1_diff < self.pos + self.v2
        && self.pos < v2_diff && v2_diff < self.pos + self.v1
    }
}

impl Collide<AABox> for RBox {
    fn collides(&self, other: &AABox) -> bool {
        let v1_diff = other.pos + self.v1 * (-self.v1.scalar(&other.pos) / self.v1.distance2());
        let v2_diff = other.pos + self.v2 * (-self.v2.scalar(&other.pos) / self.v2.distance2());
        let other_size = other.pos + other.size;
        let v1_diff_size = other_size + self.v1 * (-self.v1.scalar(&other_size) / self.v1.distance2());
        let v2_diff_size = other_size + self.v2 * (-self.v2.scalar(&other_size) / self.v2.distance2());

        self.pos < v1_diff + other.size && v1_diff < self.pos + self.v2
        && self.pos < v2_diff + other.size && v2_diff < self.pos + self.v1
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
        fn test_not_collide_aabox_aabox() {
             let a = Vec2{x: 1.0, y: 7.5};
             let b = Vec2{x: 3.0, y: 2.5};
             let c = Vec2{x: 0.5, y: 5.0};
             let aa_box = AABox{pos: a, size: b};
             let a = Vec2{x: 1.0, y: 7.5};
             let b = Vec2{x: 3.0, y: 2.5};
             let c = Vec2{x: 0.5, y: 5.0};
             let aa_box = AABox{pos: a, size: b};

             assert!(!(aa_box.collides(&c)));
        }

        #[test]
        fn test_collide_Rbox_dot() {
             let a = Vec2{x: 1.0, y: 2.5};
             let b = Vec2{x: 3.0, y: 7.5};
             let c = Vec2{x: 1.5, y: 5.0};
             let aa_box = AABox{pos: a, size: b};

             assert!(aa_box.collides(&c));
        }
    
        #[test]
        fn test_not_collide_Rbox_dot() {
             let a = Vec2{x: 1.0, y: 7.5};
             let b = Vec2{x: 3.0, y: 2.5};
             let c = Vec2{x: 0.5, y: 5.0};
             let aa_box = AABox{pos: a, size: b};

             assert!(!(aa_box.collides(&c)));
        }
}
