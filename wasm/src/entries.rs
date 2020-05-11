//! This module contains the entry points callable from javascript

use crate::context;
use crate::game_context::GameContext;
use crate::mem;
use crate::mem::get_double_buffer;
use crate::state::State;
use crate::wasm_log::init_panic_handler;
use crate::wasm_log::WasmLog;
use parking_lot::Mutex;

static IS_INIT: Mutex<bool> = Mutex::new(false);
static LOGGER: WasmLog = WasmLog;

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
extern "C" fn init(heap_base: i32) {
    let mut init = IS_INIT.lock();
    if !*init {
        unsafe {
            let heap_base = mem::MemoryAdresses::write_at(heap_base as u32);
            wee_alloc::init_ptr(heap_base as *mut u8, mem::HEAP_SIZE as usize);
        }
        context::set_context(
            context::Context::new()
                .map_err(|e| panic!("{}", e))
                .unwrap(),
        );
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        init_panic_handler();
        *init = true;
    }
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[export_name = "run_logic"]
extern "C" fn run_main_loop() {
    reset_state();
    let mut game = GameContext::new().unwrap_or_else(|e| panic!("{}", e));

    log::info!("send memory offsetst");
    crate::message_queue::Outbound::Memory((*(mem::MEM_ADDRS.read())).clone()).send();
    log::info!("send memory offsetst");

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
