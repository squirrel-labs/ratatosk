#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl std::cmp::PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(if self.x < other.x && self.y < other.y {
            std::cmp::Ordering::Less
        } else if self.x > other.x && self.y > other.y {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        })
    }
}

impl std::cmp::PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < 1e-8
        && f32::abs(self.y - other.y) < 1e-8
    }
}

impl std::cmp::Eq for Vec2 {}

impl Vec2 {
    pub fn distance(&self) -> f32 {
        f32::sqrt(self.distance2())
    }

    pub fn distance2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

pub struct Box {
    pub pos: Vec2,
    /// the size may not be smaller than zero
    pub size: Vec2, 
}

impl std::ops::Add<Vec2> for Box {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            p1: self.p1 + other,
            p2: self.p2 + other,
        }
    }
}

impl std::ops::Sub<Vec2> for Box {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            size: self.size + other
        }
    }
}

pub struct RBox {
    /// Point 1
    pub p1: Vec2,
    /// Point 2
    pub p2: Vec2, 
    /// Width
    pub w: f32,
}

impl std::ops::Add<Vec2> for RBox {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            p1: self.p1 + other,
            p2: self.p2 + other,
            w: self.w,
        }
    }
}

impl std::ops::Sub<Vec2> for RBox {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            p1: self.p1 + other,
            p2: self.p2 + other,
            w: self.w,
        }
    }
}
