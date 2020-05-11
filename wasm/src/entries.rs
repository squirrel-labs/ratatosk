//! This module contains the entry points callable from javascript

use crate::context;
use crate::game_context::GameContext;
use crate::mem;
use crate::message_queue::Outbound;
use crate::wasm_log::init_panic_handler;
use crate::wasm_log::WasmLog;

static LOGGER: WasmLog = WasmLog;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn wait_for_main_thread_notify() {
    unsafe { mem::SynchronizationMemory::get_mut() }.wait_for_main_thread_notify()
}

/// This function initializes the heap
/// #Safety
/// This function may only be called once at the start of the program
/// Any call to alloc prior to this functions invocation results in an error
#[export_name = "init"]
extern "C" fn init(heap_base: i32) {
    unsafe {
        mem::MemoryAdresses::init(heap_base as u32);
        wee_alloc::init_ptr(
            mem::MEM_ADDRS.read().heap_base as *mut u8,
            mem::HEAP_SIZE as usize,
        );
    }
    context::set_context(
        context::Context::new()
            .map_err(|e| panic!("{}", e))
            .unwrap(),
    );
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Error);
    init_panic_handler();
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[export_name = "run_logic"]
extern "C" fn run_main_loop() {
    let mut game = GameContext::new().unwrap_or_else(|e| panic!("{}", e));

    log::info!("send memory offsetst");
    Outbound::Memory((*(mem::MEM_ADDRS.read())).clone()).send();

    log::info!("syc: {}", mem::MEM_ADDRS.read().synchronization_memory);
    loop {
        game.tick().map_err(|e| panic!("{}", e)).unwrap();
        log::trace!("wait_for_main_thread_notify()");
        wait_for_main_thread_notify();
    }
}

/// This function is being exposed to javascript
#[export_name = "draw_frame"]
pub fn draw_frame() {
    context::context_mut()
        .render()
        .unwrap_or_else(|e| log::error!("{}", e));
}
