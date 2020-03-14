//! This crate is a game engine library providing many needed functions.
//! This includes math utilities such as vectors and matrices, a trait for collisions and multiple
//! boxes for modeling objects.

pub mod boxes;
pub mod collide;
pub mod math;
pub mod world;

use std::error::Error;
use std::fmt::{self, Display};

/// The error type used by the game engine.
#[derive(Debug)]
pub enum EngineError {}

impl Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Error for EngineError {}
