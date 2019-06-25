use super::matrix::Matrix;

impl std::ops::Mul for Matrix {
    type Output = Self;

    /// Laderman Algorithm
    /// see:
    /// http://www.ams.org/journals/bull/1976-82-01/S0002-9904-1976-13988-2/S0002-9904-1976-13988-2.pdf
    fn mul(self, other: Matrix) -> Matrix {
        let a = move |y: usize, x: usize| self.raw[y + 3*x - 4];
        let b = move |y: usize, x: usize| other.raw[y + 3*x - 4];

        let m1 = (a(1,1) + a(1,2) + a(1,3) - a(2,1) - a(2,2) - a(3,2) - a(3,3)) * b(2,2);
        let m2 = (a(1,1) - a(2,1)) * (b(2,2) - b(1,2));
        let m3 = a(2,2) * (-b(1,1) + b(1,2) + b(2,1) - b(2,2) - b(2,3) - b(3,1) + b(3,3));
        let m4 = (-a(1,1) + a(2,1) + a(2,2)) * (b(1,1) - b(1,2) + b(2,2));
        let m5 = (a(2,1) + a(2,2)) * (b(1,2) - b(1,1));
        let m6 = a(1,1) * b(1,1);
        let m7 = (-a(1,1) + a(3,1) + a(3,2)) * (b(1,1) - b(1,3) + b(2,3));
        let m8 = (-a(1,1) + a(3,1)) * (b(1,3) - b(2,3));
        let m9 = (a(3,1) + a(3,2)) * (-b(1,1) + b(1,3));
        let m10 = (a(1,1) + a(1,2) + a(1,3) - a(2,2) - a(2,3) - a(3,1) - a(3,2)) * b(2,3);
        let m11 = a(3,2) * (-b(1,1) + b(1,3) + b(2,1)
                            - b(2,2) - b(2,3) - b(3,1) + b(3,2));
        let m12 = (-a(1,3) + a(3,2) + a(3,3)) * (b(2,2) + b(3,1) - b(3,2));
        let m13 = (a(1,3) - a(3,3)) * (b(2,2) - b(3,2));
        let m14 = a(1,3) * b(3,1);
        let m15 = (a(3,2) + a(3,3)) * (-b(3,1) + b(3,2));
        let m16 = (-a(1,3) + a(2,2) + a(2,3)) * (b(2,3) + b(3,1) - b(3,3));
        let m17 = (a(1,3) - a(2,3)) * (b(2,3) - b(3,3));
        let m18 = (a(2,2) + a(2,3)) * (-b(3,1) + b(3,3));
        let m19 = a(1,2) * b(2,1);
        let m20 = a(2,3) * b(3,2);
        let m21 = a(2,1) * b(1,3);
        let m22 = a(3,1) * b(1,2);
        let m23 = a(3,3) * b(3,3);

        Matrix::new(
            m6 + m14 + m19,
            m1 + m4 + m5 + m6 + m12 + m14 + m15,
            m6 + m7 + m9 + m10 + m14 + m16 + m18,
            m2 + m3 + m4 + m6 + m14 + m16 + m17,
            m2 + m4 + m5 + m6 + m20,
            m14 + m16 + m17 + m18 + m21,
            m6 + m7 + m8 + m11 + m12 + m13 + m14,
            m12 + m13 + m14 + m15 + m22,
            m6 + m7 + m8 + m9 + m23)
    }
}
