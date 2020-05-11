//! This crate contains the game logic, network connections and input processing.
//! It interacts with the graphics crate through the shared array buffer

#![feature(allocator_api)]
#![feature(stdsimd)]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]

mod context;
mod entries;
mod error;
mod game_context;
mod graphics;
mod mem;
mod message_queue;
mod render;
//mod shader;
mod sprite;
mod state;
mod wasm_log;
use crate::state::State;
use parking_lot::Mutex;

static DOUBLE_BUFFER: Mutex<State> = Mutex::new(State::empty());
