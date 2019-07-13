use webhogg_wasm_shared::{
    alloc::{Allocator, LogicAllocator},
    get_allocator,
};

#[global_allocator]
static ALLOCATOR: Allocator<LogicAllocator> = get_allocator!();

mod entries;
