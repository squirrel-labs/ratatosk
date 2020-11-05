#[cfg(target_arch = "wasm32")]
use crate::mem::{atomic_read_i32, wait_until_wake_up_at};

pub use rask_engine::network::GameState;

#[repr(align(4))]
#[repr(C)]
#[derive(Debug)]
/// The SynchronizationMemory is the main interface to the `main.js`.
/// It exposes some variables from the `main.js` and is used for the atomic wait cycle.
pub struct SynchronizationMemory {
    /// Time elapsed since logic thread initialisation in milliseconds.
    pub elapsed_ms: i32,
    /// Current mouse position.
    pub mouse: (i32, i32),
    /// Canvas size in px.
    pub canvas_size: (u32, u32),
    pub player: GameState,
    pub other: GameState,
    last_elapsed_ms: i32,
}

#[allow(clippy::while_immutable_condition)]
/// The synchronization memory is a direct memory interface to the `main.js`.
/// It is updated before each logic cycle.
impl SynchronizationMemory {
    pub const fn new() -> Self {
        Self {
            elapsed_ms: 0,
            mouse: (0, 0),
            canvas_size: (0, 0),
            player: GameState::new(),
            other: GameState::new(),
            last_elapsed_ms: 0,
        }
    }

    /// This functions lets the thread sleep until it is woken up by the `main.js` or a timeout is
    /// reached.
    pub fn wait_for_main_thread_notify(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("atomic wait is not supported for non wasm targets");

        #[cfg(target_arch = "wasm32")]
        {
            self.last_elapsed_ms = self.elapsed_ms;
            while self.last_elapsed_ms
                == unsafe { atomic_read_i32(self as *const SynchronizationMemory as *const i32) }
            {
                unsafe { wait_until_wake_up_at(self as *mut SynchronizationMemory as *mut i32) }
            }
        }
    }
}
