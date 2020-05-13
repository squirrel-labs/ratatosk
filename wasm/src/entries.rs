//! This module contains the entry points callable from javascript
//! # Usage
//! To initialize the memory correctly, the exports have to be called in the follwing order:
//! 1. exports.__wasm_init_memory()
//! 2. exports.__wasm_init_tls()
//! 3. exports.init()
//! only now may other fuctions get called.
//! Calling 1-3 more than once is undefined behavior
//! When executing `init()` a message is sent to the main thread, signaling the initiaisation has
//! finished. This signal is used to start the graphics worker.

use crate::graphics::context;
use crate::logic::GameContext;
#[cfg(target_arch = "wasm32")]
use crate::{
    communication::{OutboundMessage, SynchronizationMemory},
    mem,
    wasm_log::{init_panic_handler, WasmLog},
};

#[cfg(target_arch = "wasm32")]
static LOGGER: WasmLog = WasmLog;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn wait_for_main_thread_notify() {
    #[cfg(target_arch = "wasm32")]
    unsafe { SynchronizationMemory::get_mut() }.wait_for_main_thread_notify()
}

/// This function initializes the heap, logger, panic handler and graphics context
/// The sizes of static elements such as the resource_table can be set in crate::mem
///
/// # Safety
/// This function may only be called once at the start of the program
/// Any call to alloc prior to this functions invocation results in an error
#[export_name = "init"]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn init(heap_base: i32) {
    unsafe {
        // Place the synchronization_memory, message_queue and resource_table at the beginning of
        // our heap. This call initializes mem::MEM_ADDRS
        mem::MemoryAdresses::init(heap_base as u32);
        wee_alloc::init_ptr(*mem::HEAP_BASE as *mut u8, mem::HEAP_SIZE as usize);
    }
    // create a new graphics context, this persists local data across `draw_frame` invocations
    context::set_context(
        context::Context::new()
            .map_err(|e| panic!("{}", e))
            .unwrap(),
    );
    log::set_logger(&LOGGER).unwrap();
    // change the loglevel to only show certain errors
    log::set_max_level(log::LevelFilter::Info);
    // set custom panic handler
    init_panic_handler();
    // send memery offsetst to the main thread -> initialize graphics
    OutboundMessage::Memory(
        *mem::SYNCHRONIZATION_MEMORY as u32,
        *mem::MESSAGE_QUEUE as u32,
        mem::MESSAGE_QUEUE_ELEMENT_COUNT,
    )
    .send();
}

/// Initialize the gamestate, communicate with
/// the graphics worker and set up networking.
/// This function is being exposed to javascript
#[export_name = "run_logic"]
pub extern "C" fn run_logic() {
    // create the logic game context
    let mut game = GameContext::new().unwrap_or_else(|e| panic!("{}", e));

    loop {
        game.tick()
            .unwrap_or_else(|e| log::error!("Error occured game_context.tick(): {:?}", e));
        log::trace!("wait_for_main_thread_notify()");
        // use wasms atomic wait instruction to sleep until waken by the main thread
        wait_for_main_thread_notify();
    }
}

/// This function is called to render each frame.
/// Most of the communication with the graphics api is done thorough calling js functions
#[export_name = "draw_frame"]
pub extern "C" fn draw_frame() {
    context::context_mut()
        .render()
        .unwrap_or_else(|e| log::error!("{}", e));
}
