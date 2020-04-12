use std::fs::File;
use std::io::prelude::*;

#[allow(non_snake_case)]
const fn KiB(n: u32) -> u32 {
    n * 1024
}
#[allow(non_snake_case)]
const fn MiB(n: u32) -> u32 {
    n * KiB(1024)
}
/// align the given address to the next 32bit
const fn align32_up(n: u32) -> u32 {
    (n + 3) & !3
}

const WORKER_NAME_VAR: &'static str = "CRATE";

/// Reserved memory
const MAX_MEORY: u32 = MiB(512);

/// The first page of memory is reserved
const STACK_ALIGNMENT: u32 = 1024 * 63;

/// The size of the stack. Its start is at address 0
const GRAPHICS_STACK_SIZE: u32 = MiB(1);
const GRAPHICS_HEAP_SIZE: u32 = MiB(1);

/// The size of the Allocator structures
const ALLOCATOR_SIZE: u32 = MiB(1);

/// Size of the internal resource library.
/// This determines the highest available id.
const RESOURCE_TABLE_SIZE: u32 = KiB(8);

/// Size of the message queue used to communicate between main.js and the logic thread
/// Its address must be exported to javascript.
const MESSAGE_QUEUE_SIZE: u32 = 64;

/// The address memory synchronization area.
/// It contains data needed for synchronization between main thread and logic thread.
/// This address must be exported to javascript.
const SYNCHRONIZATION_MEMORY_SIZE: u32 = 32;

/// Number of sprites to store in the double buffer
const BUFFER_SIZE: u32 = KiB(1);

fn main() -> std::io::Result<()> {
    println!("{:#?}", std::env::vars().collect::<Vec<_>>());
    let name = std::env::var(WORKER_NAME_VAR);
    let is_logic = match name {
        Ok(worker) if &worker == "logic" => true,
        Ok(worker) if &worker == "graphics" => false,
        Ok(key) => panic!(
            "{} is no valid value for {}. Possible values are logic and graphics",
            key, WORKER_NAME_VAR,
        ),
        Err(std::env::VarError::NotPresent) => {
            panic!("{} is not defined in the environment.", WORKER_NAME_VAR)
        }
        Err(err) => panic!("env var parsing failed (\"{:?}\")", err),
    };

    let graphics_stack = align32_up(STACK_ALIGNMENT + GRAPHICS_STACK_SIZE);
    let alloc = align32_up(graphics_stack);
    let graphics_heap = align32_up(alloc + ALLOCATOR_SIZE);
    let sync = align32_up(alloc + GRAPHICS_HEAP_SIZE);
    let table = align32_up(sync + SYNCHRONIZATION_MEMORY_SIZE);
    let buffer = align32_up(table + RESOURCE_TABLE_SIZE);
    let queue = align32_up(buffer + BUFFER_SIZE);
    let logic_heap = align32_up(queue + MESSAGE_QUEUE_SIZE);

    println!("cargo:rustc-env=GRAPHICS_STACK={}", graphics_stack);
    println!("cargo:rustc-env=ALLOCATOR={}", alloc);
    println!("cargo:rustc-env=GRAPHICS_HEAP={}", graphics_heap);
    println!("cargo:rustc-env=SYNCHRONIZATION_MEMORY={}", sync);
    println!("cargo:rustc-env=RESOURCE_TABLE={}", table);
    println!("cargo:rustc-env=RESOURCE_TABLE_SIZE={}", buffer - table);
    println!("cargo:rustc-env=DOUBLE_BUFFER={}", buffer);
    println!("cargo:rustc-env=DOUBLE_BUFFER_SIZE={}", queue - buffer);
    println!("cargo:rustc-env=MESSAGE_QUEUE={}", queue);
    println!("cargo:rustc-env=MESSAGE_QUEUE_SIZE={}", logic_heap - queue);
    println!("cargo:rustc-env=LOGIC_HEAP={}", logic_heap);

    if !is_logic {
        println!("cargo:rustc-cdylib-link-arg=--stack-first");
        println!(
            "cargo:rustc-cdylib-link-arg=-zstack-size={}",
            graphics_stack
        );
    };
    println!("cargo:rustc-cdylib-link-arg=--max-memory={}", MAX_MEORY);

    let out_dir = std::env::var("MEM_GEN").unwrap();
    let mut file = File::create(format!("{}", out_dir))?;
    write!(
        &mut file,
        "var memoryParameters = {{max_memory:{},queue_start:{},sync_area:{}}}",
        MAX_MEORY, queue, sync
    )?;
    Ok(())
}
