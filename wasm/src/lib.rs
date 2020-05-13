//! This crate contains two applications:
//!
//!      _________________
//!     |  main.js [0]    |
//!     |-----------------|
//!     | websockets      |
//!     | user input      |
//!     |_________________|
//!         |           A
//!         |           |
//!         | Message   | Post Message     [offscreen canvas] [7]
//!         |  Queue[4] | [5]                      A
//!         V           |                          | draw()
//!      _________________                  _________________
//!     |  logic [1]      |                |  grapics [3]    |
//!     |---------------- | "double buffer"|-----------------|
//!     | input handling  |--------------->| manage textures |
//!     | game engine     |     [6]        | call webgl      |
//!     |_________________|                |_________________|
//!
//!
//!
//! It interacts with the graphics crate through the shared array buffer

#![feature(allocator_api)]
#![feature(stdsimd)]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#[macro_use]
extern crate lazy_static;

pub mod communication;
pub mod entries;
pub mod error;
pub mod graphics;
pub mod logic;
pub mod mem;
pub mod wasm_log;