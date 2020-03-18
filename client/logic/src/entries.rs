//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::state::State;
use rask_wasm_shared::wasm_log::WasmLog;
use crate::game_context::GameContext;

fn reset_state() {
    let mut writer = get_double_buffer().borrow_writer();
    writer.set(State::default());
}

fn wait_for_main_thread_notify() {
    unsafe { rask_wasm_shared::mem::SynchronizationMemory::get_mut() }
        .wait_for_main_thread_notify()
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn run_main_loop() {
    unsafe {
        rask_wasm_shared::alloc::reset_heap(&crate::ALLOCATOR, log::LevelFilter::Debug);
    }

    log::info!("logic entry reached");
    reset_state();
    let mut game = GameContext::new()
        .map_err(|e| log::error!("{}", e))
        .unwrap();

    loop {
        game.tick()
            .map_err(|e| log::error!("{}", e))
            .unwrap();
        wait_for_main_thread_notify();
    }
}
