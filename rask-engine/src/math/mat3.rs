use std::ops;

use crate::math::Vec3;

/// A 3x3 matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    /// The elements of the matrix.
    /// [a, d, g
    ///  b, e, h
    ///  c, f, i]
    data: [f32; 9],
}

impl ops::Add for Mat3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.data[0] + other.data[0],
            self.data[3] + other.data[3],
            self.data[6] + other.data[6],
            self.data[1] + other.data[1],
            self.data[4] + other.data[4],
            self.data[7] + other.data[7],
            self.data[2] + other.data[2],
            self.data[5] + other.data[5],
            self.data[8] + other.data[8],
        )
    }
}

impl ops::AddAssign for Mat3 {
    fn add_assign(&mut self, other: Self) {
        self.data[0] += other.data[0];
        self.data[6] += other.data[6];
        self.data[3] += other.data[3];
        self.data[1] += other.data[1];
        self.data[4] += other.data[4];
        self.data[7] += other.data[7];
        self.data[2] += other.data[2];
        self.data[5] += other.data[5];
        self.data[8] += other.data[8];
    }
}

impl ops::Sub for Mat3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.data[0] - other.data[0],
            self.data[3] - other.data[3],
            self.data[6] - other.data[6],
            self.data[1] - other.data[1],
            self.data[4] - other.data[4],
            self.data[7] - other.data[7],
            self.data[2] - other.data[2],
            self.data[5] - other.data[5],
            self.data[8] - other.data[8],
        )
    }
}

impl ops::SubAssign for Mat3 {
    fn sub_assign(&mut self, other: Self) {
        self.data[0] -= other.data[0];
        self.data[1] -= other.data[1];
        self.data[2] -= other.data[2];
        self.data[3] -= other.data[3];
        self.data[4] -= other.data[4];
        self.data[5] -= other.data[5];
        self.data[6] -= other.data[6];
        self.data[7] -= other.data[7];
        self.data[8] -= other.data[8];
    }
}

impl ops::Neg for Mat3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(
            -self.data[0],
            -self.data[3],
            -self.data[6],
            -self.data[1],
            -self.data[4],
            -self.data[7],
            -self.data[2],
            -self.data[5],
            -self.data[8],
        )
    }
}

impl ops::Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self::new(
            self.data[0] * scale,
            self.data[3] * scale,
            self.data[6] * scale,
            self.data[1] * scale,
            self.data[4] * scale,
            self.data[7] * scale,
            self.data[2] * scale,
            self.data[5] * scale,
            self.data[8] * scale,
        )
    }
}

impl ops::MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, scale: f32) {
        self.data[0] *= scale;
        self.data[3] *= scale;
        self.data[6] *= scale;
        self.data[1] *= scale;
        self.data[4] *= scale;
        self.data[7] *= scale;
        self.data[2] *= scale;
        self.data[5] *= scale;
        self.data[8] *= scale;
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.data[0] * other.x() + self.data[3] * other.y() + self.data[6] * other.z(),
            self.data[1] * other.x() + self.data[4] * other.y() + self.data[7] * other.z(),
            self.data[2] * other.x() + self.data[5] * other.y() + self.data[8] * other.z(),
        )
    }
}

impl ops::Mul for Mat3 {
    type Output = Self;

    /// Laderman Algorithm
    /// see:
    /// http://www.ams.org/journals/bull/1976-82-01/S0002-9904-1976-13988-2/S0002-9904-1976-13988-2.pdf
    fn mul(self, b: Self) -> Self::Output {
        let a = move |y: usize, x: usize| self.data[y + 3 * x - 4];
        let b = move |y: usize, x: usize| b.data[y + 3 * x - 4];

        let m1 = (a(1, 1) + a(1, 2) + a(1, 3) - a(2, 1) - a(2, 2) - a(3, 2) - a(3, 3)) * b(2, 2);
        let m2 = (a(1, 1) - a(2, 1)) * (b(2, 2) - b(1, 2));
        let m3 = a(2, 2) * (-b(1, 1) + b(1, 2) + b(2, 1) - b(2, 2) - b(2, 3) - b(3, 1) + b(3, 3));
        let m4 = (-a(1, 1) + a(2, 1) + a(2, 2)) * (b(1, 1) - b(1, 2) + b(2, 2));
        let m5 = (a(2, 1) + a(2, 2)) * (b(1, 2) - b(1, 1));
        let m6 = a(1, 1) * b(1, 1);
        let m7 = (-a(1, 1) + a(3, 1) + a(3, 2)) * (b(1, 1) - b(1, 3) + b(2, 3));
        let m8 = (-a(1, 1) + a(3, 1)) * (b(1, 3) - b(2, 3));
        let m9 = (a(3, 1) + a(3, 2)) * (-b(1, 1) + b(1, 3));
        let m10 = (a(1, 1) + a(1, 2) + a(1, 3) - a(2, 2) - a(2, 3) - a(3, 1) - a(3, 2)) * b(2, 3);
        let m11 = a(3, 2) * (-b(1, 1) + b(1, 3) + b(2, 1) - b(2, 2) - b(2, 3) - b(3, 1) + b(3, 2));
        let m12 = (-a(1, 3) + a(3, 2) + a(3, 3)) * (b(2, 2) + b(3, 1) - b(3, 2));
        let m13 = (a(1, 3) - a(3, 3)) * (b(2, 2) - b(3, 2));
        let m14 = a(1, 3) * b(3, 1);
        let m15 = (a(3, 2) + a(3, 3)) * (-b(3, 1) + b(3, 2));
        let m16 = (-a(1, 3) + a(2, 2) + a(2, 3)) * (b(2, 3) + b(3, 1) - b(3, 3));
        let m17 = (a(1, 3) - a(2, 3)) * (b(2, 3) - b(3, 3));
        let m18 = (a(2, 2) + a(2, 3)) * (-b(3, 1) + b(3, 3));
        let m19 = a(1, 2) * b(2, 1);
        let m20 = a(2, 3) * b(3, 2);
        let m21 = a(2, 1) * b(1, 3);
        let m22 = a(3, 1) * b(1, 2);
        let m23 = a(3, 3) * b(3, 3);

        println!("{:?}", (m12, m15));
        println!("{:?}", (-b(3, 1), b(3, 2)));

        Self::new(
            m6 + m14 + m19,
            m1 + m4 + m5 + m6 + m12 + m14 + m15,
            m6 + m7 + m9 + m10 + m14 + m16 + m18,
            m2 + m3 + m4 + m6 + m14 + m16 + m17,
            m2 + m4 + m5 + m6 + m20,
            m14 + m16 + m17 + m18 + m21,
            m6 + m7 + m8 + m11 + m12 + m13 + m14,
            m12 + m13 + m14 + m15 + m22,
            m6 + m7 + m8 + m9 + m23,
        )
    }
}

impl ops::MulAssign for Mat3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::Div<f32> for Mat3 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self::new(
            self.data[0] / scale,
            self.data[3] / scale,
            self.data[6] / scale,
            self.data[1] / scale,
            self.data[4] / scale,
            self.data[7] / scale,
            self.data[2] / scale,
            self.data[5] / scale,
            self.data[8] / scale,
        )
    }
}

impl ops::DivAssign<f32> for Mat3 {
    fn div_assign(&mut self, scale: f32) {
        self.data[0] /= scale;
        self.data[3] /= scale;
        self.data[6] /= scale;
        self.data[1] /= scale;
        self.data[4] /= scale;
        self.data[7] /= scale;
        self.data[2] /= scale;
        self.data[5] /= scale;
        self.data[8] /= scale;
    }
}

impl Mat3 {
    /// Creates a new Mat3
    /// of the form:
    /// (a b c)
    /// (d e f)
    /// (g h i)
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> Self {
        Self {
            data: [a, d, g, b, e, h, c, f, i],
        }
    }

    /// Creates a new Mat3 from three Vec3.
    pub fn from_vec3(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Self::new(
            v1.x(),
            v2.x(),
            v3.x(),
            v1.y(),
            v2.y(),
            v3.y(),
            v1.z(),
            v2.z(),
            v3.z(),
        )
    }

    /// Returns the zero matrix.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Returns the identity matrix.
    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }

    /// Returns a matrix that rotates by `angle`.
    pub fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(cos, -sin, 0.0, sin, cos, 0.0, 0.0, 0.0, 1.0)
    }

    /// Returns a matrix that scales by `sx and sy`.
    pub fn scaling(sx: f32, sy: f32) -> Self {
        Self::new(sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0)
    }

    /// Returns a matrix that translates by (`x`, `y`).
    pub fn translation(x: f32, y: f32) -> Self {
        Self::new(1.0, 0.0, x, 0.0, 1.0, y, 0.0, 0.0, 1.0)
    }

    /// Returns the transposed matrix.
    pub fn transpose(self) -> Self {
        Self::new(
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
            self.data[6],
            self.data[7],
            self.data[8],
        )
    }
}

impl std::convert::AsRef<[f32; 9]> for Mat3 {
    fn as_ref(&self) -> &[f32; 9] {
        &self.data
    }
}
