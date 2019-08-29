use rask_wasm_shared::{
    alloc::{Allocator, GraphicsAllocator},
    get_allocator,
};

#[global_allocator]
static ALLOCATOR: Allocator<GraphicsAllocator> = get_allocator!();

mod context;
mod entries;
mod graphics;
mod render;
mod shader;
