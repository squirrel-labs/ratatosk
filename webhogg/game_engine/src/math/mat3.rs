use std::ops;

use crate::math::Vec3;

/// A 3x3 matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    data: [f32; 9],
}

/*
impl ops::Add for Mat3 {}

impl ops::AddAssign for Mat3 {}

impl ops::Sub for Mat3 {}

impl ops::SubAssign for Mat3 {}

impl ops::Neg for Mat3 {}

impl ops::Mul<f32> for Mat3 {}

impl ops::MulAssign<f32> for Mat3 {}

impl ops::Mul<Vec3> for Mat3 {}

impl ops::MulAssign<Vec3> for Mat3 {}

impl ops::Mul for Mat3 {}

impl ops::MulAssign for Mat3 {}

impl ops::Div<f32> for Mat3 {}

impl ops::DivAssign<f32> for Mat3 {}

impl Mat3 {
    pub fn zero() -> Self {}

    pub fn identity() -> Self {}

    pub fn transpose(self) -> Self {}
}
*/
