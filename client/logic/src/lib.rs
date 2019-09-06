//! This crate contains the game logic, network connections and input processing.
//! It interacts with the graphics crate through the shared array buffer

#![feature(async_await)]
#![feature(allocator_api)]

use rask_wasm_shared::{
    alloc::{SimpleAllocator, Allocator, settings::Logic},
    create_allocator,
    wee_alloc
};

#[global_allocator]
static ALLOCATOR: Allocator<SimpleAllocator, Logic> = create_allocator!();

pub mod entries;
pub mod game_context;
pub mod websocket;
