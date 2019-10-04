//! This module provides basic mathematical types and functions used in the game.
//! This includes 2- and 3-dimensional vectors as well as 2x2 and 3x3 matrices together with useful
//! operations on those types.

pub mod mat2;
pub mod mat3;
pub mod vec2;
pub mod vec3;

pub use mat2::Mat2;
pub use mat3::Mat3;
pub use vec2::Vec2;
pub use vec3::Vec3;

/// The epsilon used for comparing f32 values for equality
pub const EPSILON: f32 = 1e-8;
