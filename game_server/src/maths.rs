pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: x + other.x,
            y: y + other.y
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: x - other.x,
            y: y - other.y
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn sub(self) -> Self {
        Self {
            x: -x,
            y: -y
        }
    }
}

impl std::cmp::PartialEq for Vec2 {
    type Output = bool;
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < 1e-8
        && f32::abs(self.y - other.y) < 1e-8
    }
}

impl std::cmp::Eq for Vec2 {}

impl Vec2 {
    pub fn distance(&self) -> f32 {
        f32::sqrt(self.distance2)
    }

    pub fn distance2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}
