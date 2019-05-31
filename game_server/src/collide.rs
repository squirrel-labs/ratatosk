use crate::maths::{Vec2, Box, RBox};

pub trait Collide<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

impl Collide<Vec2> for Vec2 {
    fn collides(&self, other: &Self) -> bool {
        self == other
    }
}

impl Collide<Vec2> for Box {
    fn collides(&self, other: &Vec2) -> bool {
        self.pos < *other && other < &(self.pos + self.size) 
    }
}

impl Collide<Box> for Box {
    fn collides(&self, other: &Self) -> bool {
        self.collides(&other.pos)
        || self.collides(&(other.pos + Vec2{x: other.size.x, y: 0.0}))
        || self.collides(&(other.pos + Vec2{x: 0.0, y: other.size.y}))
        || self.collides(&(other.pos + other.size))

        || other.collides(&(self.pos))
        || other.collides(&(self.pos + Vec2{x: self.size.x, y: 0.0}))
        || other.collides(&(self.pos + Vec2{x: 0.0, y: self.size.y}))
        || other.collides(&(self.pos + self.size))
    }
}

impl Collide<Vec2> for RBox {
    fn collides(&self, other: &Vec2) -> bool {
        let mut dx = self.size.x;
        let mut dy = self.size.y;
        let len = self.size.distance();
        dx /= len;
        dy /= len;

        let dax = other.x - self.pos.x;
        let day = other.y - self.pos.y;
        
        let dot = dax * dx + day * dy;
        let px = self.pos.x + dx * dot;
        let py = self.pos.y + dy * dot;
       
        let p = Vec2{x: px, y: py};
        if !(self.pos < p && p < self.pos + self.size) {
            return false; 
        } 

        let ddx = other.x-px; 
        let ddy = other.y-py;
        let manhattenDistance = ddx + ddy;

        manhattenDistance < self.w 
    }
}

impl Collide<Box> for RBox {
    fn collides(&self, other: &Box) -> bool {
        self.collides(&other.pos)
        || self.collides(&(other.pos + Vec2{x: other.size.x, y: 0.0}))
        || self.collides(&(other.pos + Vec2{x: 0.0, y: other.size.y}))
        || self.collides(&(other.pos + other.size))

        || other.collides(&(self.pos))
        || other.collides(&(self.pos + Vec2{x: self.size.x, y: 0.0}))
        || other.collides(&(self.pos + Vec2{x: 0.0, y: self.size.y}))
        || other.collides(&(self.pos + self.size))
        
    }
}
