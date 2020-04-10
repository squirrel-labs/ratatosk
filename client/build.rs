const fn KiB(n: usize) -> usize {
    n * 1024
}
const fn MiB(n: usize) -> usize {
    n * KiB(1024)
}

const WEBWORKER_VAR: &'static str = "WEBWORKER"; 

/// Reserved memory
const MAX_MEORY: usize = 1073741824;

/// The first page of memory is reserved
const STACK_ALIGNMENT: usize = 1024 * 63;

/// The size of the stack. Its start is at address 0
const GRAPHICS_STACK_SIZE: usize = MiB(4);
const GRAPHICS_HEAP_SIZE: usize = MiB(1);

/// The size of the Allocator structures
const ALLOCATOR_SIZE: usize = MiB(1);

/// Size of the internal resource library.
/// This determines the highest available id.
const CATALOG_SIZE: usize = 512;

/// Size of rask_engine::resources::resource 
const RESOURCE_SIZE: usize = 32;

/// Length of the message queue used to communicate between main.js and the logic thread
/// This address must be exorted to javascript.
const MESSAGE_QUEUE_LENGTH: usize = 64;
const MESSAGE_QUEUE_ELEMENT_SIZE: usize = 32;

/// The address memory synchronization area.
/// It contains data needed for synchronization between main thread and logic thread.
/// This address must be exorted to javascript.
const SYNCHRONIZATION_MEMORY_SIZE: usize = 32;

/// Number of sprites to store in the double buffer
const BUFFER_SPRITE_COUNT: usize = 32;
/// Size of each sprites
const BUFFER_SPRITE_SIZE: usize = 32;


fn main() -> std::io::Result<()> {
    let logic = None;
    match std::env::var_os(WEBWORKER_VAR).map(|x|x.to_str().expect(format!("Failed to parse {}", WEBWORKER_VAR).as_str())) {
        Some("logic") => logic = Some(true),
        Some("graphics") => logic = Some(false),
        Some(key) => println!("{} is no valid value. Possibel values are logic and graphics", key),
        None => println!("{} is not defined in the environment.", WEBWORKER_VAR),
    }
    
    if logic.is_none() {
        std::process::exit(1)
    }

    let graphics_stack = STACK_ALIGNMENT + GRAPHICS_STACK_SIZE;
    let alloc = graphics_stack;
    let graphics_heap = alloc + ALLOCATOR_SIZE; 
    let sync  = alloc + GRAPHICS_HEAP_SIZE;
    let catalog = sync + SYNCHRONIZATION_MEMORY_SIZE;
    let buffer = catalog + RESOURCE_SIZE * CATALOG_SIZE;
    let queue = buffer + BUFFER_SPRITE_SIZE * BUFFER_SPRITE_COUNT;
    let logic_heap = queue + MESSAGE_QUEUE_ELEMENT_SIZE * MESSAGE_QUEUE_LENGTH;
    let logic_stack = MAX_MEORY;

    println!("cargo:rustc-env=GRAPHICS_STACK={}", graphics_stack);
    println!("cargo:rustc-env=ALLOCATOR={}", alloc);
    println!("cargo:rustc-env=GRAPHICS_HEAP={}", graphics_heap);
    println!("cargo:rustc-env=SYNCHRONIZATION_MEMORY={}", sync);
    println!("cargo:rustc-env=CATALOG={}", catalog);
    println!("cargo:rustc-env=DOUBLE_BUFFER={}", catalog);
    println!("cargo:rustc-env=MESSAGE_QUEUE={}", queue);
    println!("cargo:rustc-env=LOGIC_HEAP={}", logic_heap);
    println!("cargo:rustc-env=LOGIC_STACK={}", logic_stack);


    let stacksize = if logic.unwrap() {logic_stack} else {graphics_stack};
    println!("cargo:rustc-flags=-Clink-arg=--stack-first -Clink-arg=--no-entry -Clink-arg=--allow-undefined -Clink-arg=--strip-all -Clink-arg=--export-dynamic -Clink-arg=--import-memory -Clink-arg=--shared-memory -Clink-arg=--max-memory={} -Clink-arg=--threads -Clink-arg=-zstack-size={} -Clink-arg=--export=__wasm_init_memory -Clink-arg=--no-check-features -Clink-arg=--export=__wasm_init_tls -Clink-arg=--export=__tls_size -Ctarget-feature=+atomics,+bulk-memory", MAX_MEORY, stacksize);

    use std::fs::File;
    use std::io::prelude::*;
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut file = File::create(format!("{}/mem.json", out_dir))?;
    file.write_all(format!("{{max_memory:{},queue_start:{},sync_area:{}}}", MAX_MEORY, queue, sync).as_bytes())?;
    Ok(())
}
