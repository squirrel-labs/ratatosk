#![feature(allocator_api)]

use rask_wasm_shared::{
    alloc::{SimpleAllocator, Allocator, Initial, NaiveInitial, settings::Graphics},
    create_allocator,
    wee_alloc
};

create_allocator!(ALLOCATOR, wee_alloc::WeeAlloc<'static>, Graphics, wee_alloc::WeeAlloc::INIT);
//create_allocator!(ALLOCATOR, SimpleAllocator, Graphics);

mod context;
mod entries;
mod graphics;
mod render;
mod shader;
