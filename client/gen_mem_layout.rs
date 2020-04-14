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
const fn align_page_up(n: u32) -> u32 {
    let x = (1<<16) - 1; 
    (n + x) & !x
}

const WORKER_NAME_VAR: &'static str = "CRATE";


/// The first page of memory is reserved
const STACK_ALIGNMENT: u32 = 1024 * 63;

/// The size of the stack. Its start is at address 0
const GRAPHICS_STACK_SIZE: u32 = MiB(100);
const GRAPHICS_HEAP_SIZE: u32 = MiB(200);
const LOGIC_HEAP_SIZE: u32 = MiB(200);
const LOGIC_STACK_SIZE: u32 = MiB(100);

/// The size of the Allocator structures
/// the size of on of the the wee_alloc structures is 2056 bytes
const ALLOCATOR_SIZE: u32 = MiB(6);

/// The address memory synchronization area.
/// It contains data needed for synchronization between main thread and logic thread.
/// This address must be exported to javascript.
const SYNCHRONIZATION_MEMORY_SIZE: u32 = 64;

/// Size of the internal resource library.
/// This determines the highest available id.
const RESOURCE_TABLE_SIZE: u32 = KiB(8);

/// Size of the message queue used to communicate between main.js and the logic thread
/// Its address must be exported to javascript.
const MESSAGE_QUEUE_SIZE: u32 = KiB(1);

/// Size of the double buffer
const BUFFER_SIZE: u32 = KiB(8);

fn main() -> std::io::Result<()> {
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

    let logic_stack = align32_up(STACK_ALIGNMENT + LOGIC_STACK_SIZE);
    let graphics_stack = align32_up(logic_stack + GRAPHICS_STACK_SIZE);
    let sync = align32_up(graphics_stack + KiB(1));
    let table = align32_up(sync + SYNCHRONIZATION_MEMORY_SIZE);
    let buffer = align32_up(table + RESOURCE_TABLE_SIZE);
    let queue = align32_up(buffer + BUFFER_SIZE);
    let alloc = align32_up(queue + MESSAGE_QUEUE_SIZE);
    let graphics_heap = align32_up(alloc + ALLOCATOR_SIZE);
    let logic_heap = align32_up(graphics_heap + GRAPHICS_HEAP_SIZE);
    let max_mem = align_page_up(logic_heap + LOGIC_HEAP_SIZE);

    println!("cargo:rustc-env=LOGIC_STACK={}", logic_stack);
    println!("cargo:rustc-env=GRAPHICS_STACK={}", graphics_stack);
    println!("cargo:rustc-env=ALLOCATOR={}", alloc);
    println!("cargo:rustc-env=GRAPHICS_HEAP={}", graphics_heap);
    println!("cargo:rustc-env=SYNCHRONIZATION_MEMORY={}", sync);
    println!("cargo:rustc-env=RESOURCE_TABLE={}", table);
    println!("cargo:rustc-env=RESOURCE_TABLE_SIZE={}", buffer - table);
    println!("cargo:rustc-env=DOUBLE_BUFFER={}", buffer);
    println!("cargo:rustc-env=DOUBLE_BUFFER_SIZE={}", queue - buffer);
    println!("cargo:rustc-env=MESSAGE_QUEUE={}", queue);
    println!("cargo:rustc-env=MESSAGE_QUEUE_SIZE={}", alloc - queue);
    println!("cargo:rustc-env=LOGIC_HEAP={}", logic_heap);

    let out_dir = std::env::var("MEM_GEN").unwrap();
    let mut file = File::create(format!("{}/mem.conf", out_dir))?;
    if is_logic {
        write!(
            &mut file,
            " -Clink-arg=-zstack-size={} -Clink-arg=--max-memory={}",
            logic_stack, max_mem
        )?;
        println!("cargo:rustc-env=WEE_ALLOC_STATIC_ARRAY_BACKEND_BYTES={}", LOGIC_HEAP_SIZE);
    } else {
        write!(
            &mut file,
            " -Clink-arg=-zstack-size={} -Clink-arg=--max-memory={}",
            graphics_stack, max_mem
        )?;
        println!("cargo:rustc-env=WEE_ALLOC_STATIC_ARRAY_BACKEND_BYTES={}", GRAPHICS_HEAP_SIZE);
    }

    let mut file = File::create(format!("{}/mem.json", out_dir))?;
    write!(
        &mut file,
        "var memoryParameters = {{max_memory:{},queue_start:{},sync_area:{}}}",
        max_mem, queue, sync
    )?;
    Ok(())
}
