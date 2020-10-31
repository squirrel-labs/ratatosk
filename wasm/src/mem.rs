//! This module abstracts some of the raw memory interaction and calculates the offsets of shared
//! memory constructs.

static mut TLS_SIZE: usize = 0;
pub const STACK_ALIGN: usize = 1024 * 64;
pub const GRAPHICS_STACK_SIZE: usize = 1024 * 64 * 8;
pub const WORKER_STACK_SIZE: usize = 1024 * 64 * 8;

/// Align given memory address up to the alignment of T.
///
/// # Example
///
/// ```
/// # use crate::rask_wasm::mem::align_up;
/// let a = align_up::<u32>(1);
/// assert_eq!(a, 4);
/// ```
pub const fn align_up<T>(addr: usize) -> usize {
    let x = std::mem::align_of::<T>() - 1;
    (addr + x) & !x
}

/// Set the size of the global thread local storage.
/// # Safety
/// This function has to be called prior to any invocaton of the alloc_tls function.
/// This function may never be called more than once.
pub unsafe fn set_tls_size(tls_size: u32) {
    TLS_SIZE = tls_size as usize;
}

pub fn alloc_tls() -> *const u8 {
    unsafe {
        let tls_layout = core::alloc::Layout::from_size_align(TLS_SIZE as usize, 8).unwrap();
        std::alloc::alloc(tls_layout)
    }
}

pub fn alloc_stack(stack_size: usize) -> *const u8 {
    unsafe {
        let stack_layout = core::alloc::Layout::from_size_align(stack_size, STACK_ALIGN).unwrap();
        std::alloc::alloc(stack_layout).offset(stack_size as isize)
    }
}

pub fn get_tls_size() -> usize {
    unsafe { TLS_SIZE }
}

/// # Safety
///
/// This function is not safe, it is a wrapper around raw pointer operations.
pub unsafe fn atomic_write_u8(ptr: *mut u8, v: u8) {
    (*(ptr as *mut core::sync::atomic::AtomicU8)).store(v, core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
///
/// This function is not safe, it is a wrapper around raw pointer operations.
pub unsafe fn atomic_read_u8(ptr: *const u8) -> u8 {
    (*(ptr as *const core::sync::atomic::AtomicU8)).load(core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
///
/// This function is not safe, it is a wrapper around raw pointer operations.
pub unsafe fn atomic_read_i32(ptr: *const i32) -> i32 {
    (*(ptr as *const core::sync::atomic::AtomicI32)).load(core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
///
/// This function is not safe, it is a wrapper around raw pointer operations.
pub unsafe fn atomic_write_u32(ptr: *mut u32, v: u32) {
    (*(ptr as *mut core::sync::atomic::AtomicU32)).store(v, core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
///
/// This function is not safe, it is a wrapper around raw pointer operations.
pub unsafe fn atomic_read_u32(ptr: *const u32) -> u32 {
    (*(ptr as *const core::sync::atomic::AtomicU32)).load(core::sync::atomic::Ordering::SeqCst)
}

#[allow(unused_variables)]
/// # Safety
///
/// This function is safe as long the thread waits at a valid memory address.
pub unsafe fn wait_until_wake_up_at(ptr: *mut i32) {
    let timeout = 5;
    #[cfg(target_arch = "wasm32")]
    {
        let res = core::arch::wasm32::memory_atomic_wait32(
            ptr,
            atomic_read_i32(ptr),
            1000 * 1000 * 1000 * timeout,
        );
        if res != 0 {
            log::trace!("Thread woke up after {}s with code {}", timeout, res);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    log::info!("atomic wait is not supported for non wasm targets");
}
