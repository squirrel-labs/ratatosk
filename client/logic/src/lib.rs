//! This crate contains the game logic, network connections and input processing.
//! It interacts with the graphics crate through the shared array buffer

#![feature(async_await)]

use webhogg_wasm_shared::{
    alloc::{Allocator, LogicAllocator},
    get_allocator,
};

#[global_allocator]
static ALLOCATOR: Allocator<LogicAllocator> = get_allocator!();

pub mod entries;
pub mod websocket;
