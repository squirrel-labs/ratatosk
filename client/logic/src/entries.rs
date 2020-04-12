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

    use rask_wasm_shared as mem;
    log::info!("logic entry reached");
    //log::info!("Allocator: {}", mem::ALLOCATOR);
    //log::info!("logic heap: {}", mem::LOGIC_HEAP);
    //log::info!("buffer: {}", mem::DOUBLE_BUFFER);
    //log::info!("graphics_heap: {}", mem::GRAPHICS_HEAP);
    //log::info!("graphics_stack: {}", mem::GRAPHICS_STACK);
    log::info!("queue: {}", mem::MESSAGE_QUEUE);
    log::info!("table: {}", mem::RESOURCE_TABLE);
    //log::info!("buffer size: {}", mem::DOUBLE_BUFFER_SIZE);
    log::info!("queue size: {}", mem::MESSAGE_QUEUE_SIZE);
    log::info!("table size: {}", mem::RESOURCE_TABLE_SIZE);
    log::info!("table count: {}", mem::RESOURCE_TABLE_ELEMENT_COUNT);
    log::info!("queue count: {}", mem::MESSAGE_QUEUE_ELEMENT_COUNT);
    log::info!("buffer count: {}", mem::DOUBLE_BUFFER_ELEMENT_COUNT);
    //log::info!("sync: {}", mem::SYNCHRONIZATION_MEMORY);
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
