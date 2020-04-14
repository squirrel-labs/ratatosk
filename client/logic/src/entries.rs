//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::game_context::GameContext;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::state::State;
use rask_wasm_shared::wasm_log::WasmLog;

fn reset_state() {
    let mut writer = get_double_buffer().borrow_writer();
    writer.set(State::default());
}

fn wait_for_main_thread_notify() {
    unsafe { rask_wasm_shared::mem::SynchronizationMemory::get_mut() }.wait_for_main_thread_notify()
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn run_main_loop() {
    unsafe {
        rask_wasm_shared::alloc::reset_heap(&crate::ALLOCATOR, log::LevelFilter::Debug);
    }

    //let b = Box::new(0u8);
    //unsafe {std::ptr::read_volatile(&b)};
    use std::alloc::GlobalAlloc;
    unsafe {
        let addr = crate::ALLOCATOR.alloc(core::alloc::Layout::from_size_align_unchecked(1, 4)) as *mut u32;
        std::ptr::read_volatile(addr);
        *addr = 0xffffffff;
        std::ptr::read_volatile(addr);
    };
    return;

    use rask_wasm_shared as mem;
    log::info!("logic entry reached");
    log::info!("logic_stack: {:#x}", mem::LOGIC_STACK);
    log::info!("graphics_stack: {:#x}", mem::GRAPHICS_STACK);
    log::info!("Allocator: {:#x}", mem::ALLOCATOR);
    log::info!("graphics_heap: {:#x}", mem::GRAPHICS_HEAP);
    log::info!("sync: {:#x}", mem::SYNCHRONIZATION_MEMORY);
    log::info!("table: {:#x}", mem::RESOURCE_TABLE);
    log::info!("buffer: {:#x}", mem::DOUBLE_BUFFER);
    log::info!("queue: {:#x}", mem::MESSAGE_QUEUE);
    log::info!("logic heap: {:#x}", mem::LOGIC_HEAP);
    log::info!("buffer size: {:#x}", mem::DOUBLE_BUFFER_SIZE);
    log::info!("queue size: {:#x}", mem::MESSAGE_QUEUE_SIZE);
    log::info!("table size: {:#x}", mem::RESOURCE_TABLE_SIZE);
    log::info!("table count: {}", mem::RESOURCE_TABLE_ELEMENT_COUNT);
    log::info!("queue count: {}", mem::MESSAGE_QUEUE_ELEMENT_COUNT);
    log::info!("buffer count: {}", mem::DOUBLE_BUFFER_SPRITE_COUNT);
    reset_state();
    let mut game = GameContext::new()
        .map_err(|e| log::error!("{}", e))
        .unwrap();

    log::info!("game_context crated");
    loop {
        game.tick().map_err(|e| log::error!("{}", e)).unwrap();
        log::info!("wait_for_main_thread_notify()");
        wait_for_main_thread_notify();
    }
}
