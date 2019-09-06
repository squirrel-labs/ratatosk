#![feature(allocator_api)]

use rask_wasm_shared::{
    alloc::{SimpleAllocator, Allocator, settings::Graphics},
    create_allocator,
    wee_alloc
};

#[global_allocator]
static ALLOCATOR: Allocator<SimpleAllocator, Graphics> = create_allocator!();

mod context;
mod entries;
mod graphics;
mod render;
mod shader;
