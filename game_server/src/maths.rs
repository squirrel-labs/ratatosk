#[derive(Clone, Copy, Debug)]
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

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
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

impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
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
        if self.x < other.x && self.y < other.y {
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

    pub fn norm(&self) -> Vec2 {
        let len = self.distance();
        Vec2 {
            x: self.x / len,
            y: self.y / len,    
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AABox {
    pub pos: Vec2,
    /// the size may not be smaller than zero
    pub size: Vec2, 
}

impl std::ops::Add<Vec2> for AABox {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            size: self.size,
        }
    }
}

impl std::ops::AddAssign<Vec2> for AABox {
    fn add_assign(&mut self, other: Vec2) {
        self.pos += other
    }
}

impl std::ops::Sub<Vec2> for AABox {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            size: self.size
        }
    }
}

impl std::ops::SubAssign<Vec2> for AABox {
    fn sub_assign(&mut self, other: Vec2) {
        self.pos -= other
    }
}

impl std::cmp::PartialEq for AABox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.size == other.size
    }
}

impl std::cmp::Eq for AABox {}

#[derive(Clone, Copy, Debug)]
pub struct RBox {
    /// Point 1
    pub pos: Vec2,
    /// Point 2
    pub size: Vec2, 
    /// Width Attention manhatten distance!!!
    pub w: f32,
}

impl std::ops::Add<Vec2> for RBox {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            size: self.size,
            w: self.w,
        }
    }
}

impl std::ops::AddAssign<Vec2> for RBox {
    fn add_assign(&mut self, other: Vec2) {
        self.pos += other
    }
}

impl std::ops::Sub<Vec2> for RBox {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            size: self.size + other,
            w: self.w,
        }
    }
}

impl std::ops::SubAssign<Vec2> for RBox {
    fn sub_assign(&mut self, other: Vec2) {
        self.pos -= other
    }
}

impl std::cmp::PartialEq for RBox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.size == other.size
            && f32::abs(self.w - other.w) < 1e-8
    }
}

impl std::cmp::Eq for RBox {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_vec2() {
        let a = Vec2{x: 1.0, y: 7.5};
        let b = Vec2{x: -3.0, y: 2.5};
        let c = Vec2{x: -2.0, y: 10.0};

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_neg_vec2() {
        let a = Vec2{x: 1.0, y: 7.5};
        let b = Vec2{x: -1.0, y: -7.5};

        assert_eq!(-a, b);
    }

    #[test]
    fn test_sub_vec2() {
        let a = Vec2{x: 1.0, y: 7.5};
        let b = Vec2{x: -3.0, y: 2.5};
        let c = Vec2{x: 4.0, y: 5.0};

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_distance_vec2() {
        let a = Vec2{x: 2.0, y: 2.0};

        assert!(f32::abs(a.distance() - 2.0) < 1e8);
    }

    #[test]
    fn test_distance2_vec2() {
        let a = Vec2{x: 1.0, y: 2.0};

        assert!(f32::abs(a.distance2() - 5.0) < 1e8);
    }
    #[test]
    fn test_norm_vec2() {
        let a = Vec2{x: 2.0, y: -2.0};
        let b = Vec2{x: std::f32::consts::FRAC_1_SQRT_2, y: -std::f32::consts::FRAC_1_SQRT_2};

        assert_eq!(a.norm(), b);
    }

    #[test]
    fn test_add_aabox_vec2() {
        let a = Vec2{x: 1.0, y: 7.5};
        let b = Vec2{x: -3.0, y: 2.5};
        let mut aa_box = AABox{pos: a, size: b};
        let bb_box = AABox{pos: a + b,size: b};
        aa_box += b;

        assert_eq!(aa_box, bb_box);
    }

    #[test]
    fn test_sub_aabox_vec2() {
        let a = Vec2{x: 1.0, y: 7.5};
        let b = Vec2{x: -3.0, y: 2.5};
        let mut aa_box = AABox{pos: a, size: b};
        let bb_box = AABox{pos: a - b,size: b};
        aa_box -= b;

        assert_eq!(aa_box, bb_box);
    }

    #[test]
    fn test_add_rbox_vec2() {
        let a = Vec2{x: 1.0, y: 7.5};
        let b = Vec2{x: -3.0, y: 2.5};
        let mut aa_box = RBox{pos: a, size: b, w: 8.0};
        let bb_box = RBox{pos: a + b,size: b, w: 8.0};
        aa_box += b;

        assert_eq!(aa_box, bb_box);
    }

    #[test]
    fn test_sub_rbox_vec2() {
        let a = Vec2{x: 1.0, y: 7.5};
        let b = Vec2{x: -3.0, y: 2.5};
        let mut aa_box = RBox{pos: a, size: b, w: 8.0};
        let bb_box = RBox{pos: a - b,size: b, w: 8.0};
        aa_box -= b;

        assert_eq!(aa_box, bb_box);
    }
}