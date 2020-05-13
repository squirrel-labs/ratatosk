#[cfg(target_arch = "wasm32")]
use crate::mem::{atomic_read_i32, wait_until_wake_up_at, SYNCHRONIZATION_MEMORY};

#[repr(C)]
/// The GameState contains data to be sent over the network and is read by main.js
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    /// Encodes actions the player takes + status effects e.g. poisoned
    pub player_state: i32,
}

impl GameState {
    pub const fn new() -> Self {
        Self {
            player_x: 0.0,
            player_y: 0.0,
            player_state: 0,
        }
    }
}

#[repr(align(4))]
#[repr(C)]
/// The SynchronizationMemory is the main interface to the main.js
/// It exposes some variables from the main.js and is used for the atomic wait cycle
pub struct SynchronizationMemory {
    /// Time elapsed since logic thread initialisation in milliseconds
    pub elapsed_ms: i32,
    /// Current mouse position
    pub mouse: (i32, i32),
    /// Canvas size in px
    pub canvas_size: (u32, u32),
    pub player: GameState,
    pub other: GameState,
    last_elapsed_ms: i32,
}

const DUMMY_SYNC: SynchronizationMemory = SynchronizationMemory::new();
#[allow(clippy::while_immutable_condition)]
/// The synchronization memory is a direct memory interface to the main.js
/// It ist uptated before each logic cycle
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
    #[cfg(target_arch = "wasm32")]
    /// # Safety
    /// This function is safe, if the SYNCHRONIZATION_MEMORY memory address is valid
    /// and is only written to using atomic operations
    pub unsafe fn get() -> &'static Self {
        &*(*SYNCHRONIZATION_MEMORY as *const Self)
    }
    #[cfg(not(target_arch = "wasm32"))]
    /// # Safety
    /// This function completely safe as it just returns dummy data on non wasm 32 targets
    pub unsafe fn get() -> &'static Self {
        &DUMMY_SYNC
    }
    #[cfg(target_arch = "wasm32")]
    /// # Safety
    /// This function is safe, if the SYNCHRONIZATION_MEMORY memory address is valid
    /// and is only written to using atomic operations
    pub unsafe fn get_mut() -> &'static mut Self {
        &mut *(*SYNCHRONIZATION_MEMORY as *mut Self)
    }

    /// This functions lets the thread sleep until it is woken up by the main.js or a timeout is
    /// reached
    pub fn wait_for_main_thread_notify(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        log::info!("atomic wait is not supported for non wasm targets");

        #[cfg(target_arch = "wasm32")]
        {
            self.last_elapsed_ms = self.elapsed_ms;
            while self.last_elapsed_ms
                == unsafe { atomic_read_i32(*SYNCHRONIZATION_MEMORY as *const i32) }
            {
                unsafe { wait_until_wake_up_at(*SYNCHRONIZATION_MEMORY as *mut i32) }
            }
        }
    }
}
