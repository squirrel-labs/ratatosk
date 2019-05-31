pub trait Collide<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

impl Collide<Vec2> for Vec2 {
    fn collides(self, other: Self) {
        self == other
    }
}

impl Collide<Vec2> for Box {
    fn collides(self, other: Vec2) {
        self.pos < other < self.pos + self.size 
    }
}

impl Collide<Box> for Box {
    fn collides(self, other: Self) {
        self.collides(other.pos) 
        || other.collides(self.pos) 
    }
}

impl Collide<Vec2> for RBox {
    fn collides(self, other: Vec2) {
         
        || other.pos < self.pos < other.pos + other.size 
    }
}

impl Collide<Box> for Box {
    fn collides(self, other: Self) {
        self.pos < other.pos < self.pos + self.size 
        || other.pos < self.pos < other.pos + other.size 
    }
}
