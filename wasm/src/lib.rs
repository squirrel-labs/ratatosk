//! This crate contains the game logic, network connections and input processing.
//! It interacts with the graphics crate through the shared array buffer

#![feature(allocator_api)]
#![feature(type_ascription)]
#![feature(stdsimd)]
#![feature(arbitrary_enum_discriminant)]

//mod alloc;
mod context;
mod double_buffer;
mod entries;
mod error;
mod game_context;
mod graphics;
mod mem;
mod message_queue;
mod render;
mod shader;
mod sprite;
mod state;
mod wasm_log;
