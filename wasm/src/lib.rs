//! This crate contains the game logic, network connections and input processing.
//! It interacts with the graphics crate through the shared array buffer

#![feature(allocator_api)]
#![feature(type_ascription)]
#![feature(stdsimd)]
#![feature(arbitrary_enum_discriminant)]

mod alloc;
mod double_buffer;
mod error;
mod game_context;
mod mem;
mod message_queue;
mod sprite;
mod state;
mod wasm_log;

use wee_alloc;

mod context;
mod entries;
mod graphics;
mod render;
mod shader;
