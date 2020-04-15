#![feature(allocator_api)]

use rask_wasm_shared::{
    alloc::{settings::Graphics, Allocator, Initial},
    create_allocator, wee_alloc,
};

create_allocator!(
    ALLOCATOR,
    wee_alloc::WeeAlloc<'static>,
    Graphics,
    wee_alloc::WeeAlloc::INIT
);

mod context;
mod entries;
mod graphics;
mod render;
mod shader;
