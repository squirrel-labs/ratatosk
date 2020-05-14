//! This crate is a game engine library providing many needed functions.
//! This includes math utilities such as vectors and matrices, a trait for collisions and multiple
//! boxes for modeling objects.

pub mod boxes;
pub mod collide;
pub mod engine;
pub mod error;
pub mod events;
pub mod math;
pub mod network;
pub mod resources;
pub mod world;

#[doc(inline)]
pub use error::EngineError;

#[doc(inline)]
pub use engine::GameEngine;

#[doc(inline)]
pub use engine::RaskEngine;
