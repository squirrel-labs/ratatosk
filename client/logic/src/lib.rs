//! This crate contains the game logic, network connections and input processing.
//! It interacts with the graphics crate through the shared array buffer

#![feature(allocator_api)]

use rask_wasm_shared::{
    alloc::{settings::Logic, Allocator, Initial, NaiveInitial, SimpleAllocator},
    create_allocator, wee_alloc,
};

create_allocator!(
    ALLOCATOR,
    wee_alloc::WeeAlloc<'static>,
    Logic,
    wee_alloc::WeeAlloc::INIT
);
//create_allocator!(ALLOCATOR, SimpleAllocator, Logic);

pub mod entries;
pub mod game_context;
pub mod websocket;
