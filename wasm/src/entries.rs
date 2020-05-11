//! This module contains the entry points callable from javascript

use crate::context;
use crate::game_context::GameContext;
use crate::mem;
use crate::mem::get_double_buffer;
use crate::state::State;
use crate::wasm_log::init_panic_handler;
use parking_lot::Mutex;

static IS_INIT: Mutex<bool> = Mutex::new(false);

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn reset_state() {
    let mut writer = get_double_buffer().borrow_writer();
    writer.set(State::default());
}

fn wait_for_main_thread_notify() {
    unsafe { mem::SynchronizationMemory::get_mut() }.wait_for_main_thread_notify()
}

#[export_name = "init"]
extern "C" fn init() {
    let mut init = IS_INIT.lock();
    if !*init {
        unsafe {
            wee_alloc::init_ptr(mem::__heap_base as *mut u8, 1024 * 64 * 16);
            log::info!("{}", mem::__heap_base);
        }
        context::set_context(
            context::Context::new()
                .map_err(|e| panic!("{}", e))
                .unwrap(),
        );
        *init = true;
    }
    init_panic_handler();
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[export_name = "run_logic"]
extern "C" fn run_main_loop() {
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

/// This function is being exposed to javascript
#[export_name = "draw_frame"]
pub fn draw_frame() {
    context::context_mut()
        .render()
        .unwrap_or_else(|e| log::error!("{}", e));
}
