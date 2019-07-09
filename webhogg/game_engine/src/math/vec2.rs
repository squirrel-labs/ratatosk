use std::ops;

/// A 2-dimensional euclidean vector.
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    /// The x coordinate.
    pub x: f32,
    /// The y coordinate.
    pub y: f32,
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self {
            x: self.x / scale,
            y: self.y / scale,
        }
    }
}

impl ops::Div for Vec2 {
    type Output = Self;

    fn div(self, other: Vec2) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::cmp::PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x <= other.x && self.y <= other.y {
            Some(std::cmp::Ordering::Less)
        } else if self.x > other.x && self.y > other.y {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl std::cmp::PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < EPSILON && f32::abs(self.y - other.y) < EPSILON
    }
}

impl std::cmp::Eq for Vec2 {}

impl Vec2 {
    /// Returns the euclidean norm of the vector.
    pub fn norm(&self) -> f32 {
        f32::hypot(self.x, self.y)
    }

    /// Returns the square of the euclidean norm of the vector.
    pub fn norm2(&self) -> f32 {
        self.dot(*self)
    }

    /// Returns the dot product.
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns a normalized version of the vector, that is, a vector that points in the same direction, but has length 1.
    pub fn normalized(&self) -> Vec2 {
        *self / self.norm()
    }
}

