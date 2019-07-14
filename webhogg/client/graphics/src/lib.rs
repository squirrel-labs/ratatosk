use webhogg_wasm_shared::{
    alloc::{Allocator, GraphicsAllocator},
    get_allocator,
};

#[global_allocator]
static ALLOCATOR: Allocator<GraphicsAllocator> = get_allocator!();

mod entries;
