use std::ops;

use crate::math::Vec2;

/// A 2x2 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat2 {
    data: [f32; 4],
}

/*
impl ops::Add for Mat2 {}

impl ops::AddAssign for Mat2 {}

impl ops::Sub for Mat2 {}

impl ops::SubAssign for Mat2 {}

impl ops::Neg for Mat2 {}

impl ops::Mul<f32> for Mat2 {}

impl ops::MulAssign<f32> for Mat2 {}

impl ops::Mul<Vec2> for Mat2 {}

impl ops::MulAssign<Vec2> for Mat2 {}

impl ops::Mul for Mat2 {}

impl ops::MulAssign for Mat2 {}

impl ops::Div<f32> for Mat2 {}

impl ops::DivAssign<f32> for Mat2 {}

impl Mat2 {
    pub fn zero() -> Self {}

    pub fn identity() -> Self {}

    pub fn transpose(self) -> Self {}
}
*/
