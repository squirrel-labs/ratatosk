//! Note: this requires `packed_simd = "0.3.3"` as dependency in Cargo.toml to compile
//! This module is supposed to have the same types and functions as the math module, but is using
//! SIMD to achieve that. However, currently the module is not exposed.

pub mod mat2;
pub mod vec2;

pub use mat2::Mat2;
pub use vec2::Vec2;

pub const EPSILON: f32 = 1e-8;
