//! This module contains the entry points callable from javascript

use crate::game_context::GameContext;
use crate::mem;
use crate::mem::get_double_buffer;
use crate::state::State;

use std::panic;
use crate::context;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn reset_state() {
    let mut writer = get_double_buffer().borrow_writer();
    writer.set(State::default());
}

fn wait_for_main_thread_notify() {
    unsafe { mem::SynchronizationMemory::get_mut() }.wait_for_main_thread_notify()
}

#[export_name = "init_alloc"]
extern "C" fn init_alloc() {
    unsafe { wee_alloc::init_ptr(crate::mem::__heap_base as *mut u8, 1024 * 64 * 16);}
    log::info!("{}", crate::mem::__heap_base);
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[export_name = "init_alloc"]
extern fn run_main_loop() {
    log::info!("table count: {}", mem::RESOURCE_TABLE_ELEMENT_COUNT);
    log::info!("queue count: {}", mem::MESSAGE_QUEUE_ELEMENT_COUNT);
    log::info!("buffer count: {}", mem::DOUBLE_BUFFER_SPRITE_COUNT);
    reset_state();
    let mut game = GameContext::new().unwrap_or_else(|e| panic!("{}", e));
    loop {
        log::info!("a");
        wait_for_main_thread_notify();
        log::info!("b");
        game.tick().map_err(|e| panic!("{}", e)).unwrap();
        log::info!("wait_for_main_thread_notify()");
    }
}

#[allow(dead_code)]
/// This function is being exposed to javascript
#[export_name = "initialise_graphics_context"]
pub fn initialise_graphics_context() {
    log::info!("graphics entry reached");

    context::set_context(
        context::Context::new()
            .map_err(|e| panic!("{}", e))
            .unwrap(),
    );
}

#[allow(dead_code)]
/// This function is being exposed to javascript
#[export_name = "draw_frame"]
pub fn draw_frame() {
    context::context_mut()
        .render()
        .unwrap_or_else(|e| log::error!("{}", e));
}
