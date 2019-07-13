use webhogg_wasm_shared::alloc::Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {
    pos: 1024 + 32,
    mem0: 0x100000,
};

mod entries;
