use webhogg_wasm_shared::alloc::Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {
    pos: 1024,
    mem0: 0x10000,
};

mod entries;
