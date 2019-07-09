use std::ops;

/// A 3-dimensional euclidean vector.
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    /// The x coordinate.
    pub x: f32,
    /// The y coordinate.
    pub y: f32,
    /// The z coordinate.
    pub z: f32,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < EPSILON && f32::abs(self.y - other.y) < EPSILON && f32::abs(self.z - other.z) < EPSILON
    }
}

impl std::cmp::Eq for Vec3 {}

impl Vec3 {
    /// Returns the euclidean norm of the vector.
    pub fn norm(&self) -> f32 {
        f32::sqrt(self.x * self.x, self.y * self.y, self.z * self.z)
    }

    /// Returns the square of the euclidean norm of the vector.
    pub fn norm2(&self) -> f32 {
        self.dot(*self)
    }

    /// Returns the dot product.
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns a normalized version of the vector, that is, a vector that points in the same direction, but has length 1.
    pub fn normalized(&self) -> Vec2 {
        *self / self.norm()
    }
}

