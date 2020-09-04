#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::{memory_atomic_notify, memory_atomic_wait32};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicI32, Ordering};

pub struct Mutex<T> {
    locked: UnsafeCell<AtomicI32>,
    value: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: UnsafeCell::new(AtomicI32::new(0)),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        MutexGuard::new(self)
    }

    #[cfg(target_arch = "wasm32")]
    fn locked_ptr(&self) -> *mut i32 {
        self.locked.get() as *mut i32
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> MutexGuard<'a, T> {
    pub fn new(mutex: &'a Mutex<T>) -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            memory_atomic_wait32(mutex.locked_ptr(), 1, -1)
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            while (unsafe { &*mutex.locked.get() }).load(Ordering::SeqCst) == 1 {}
        }
        (unsafe { &*mutex.locked.get() }).store(1, Ordering::SeqCst);
        Self { mutex }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        (unsafe { &*self.mutex.locked.get() }).store(0, Ordering::SeqCst);
        #[cfg(target_arch = "wasm32")]
        unsafe {
            memory_atomic_notify(self.mutex.locked_ptr(), 1)
        };
    }
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.value.get() }
    }
}

/// # Safety
/// This implementation is only guaranteed to be thread-safe, if there are only 2 threads using
/// the RwLocks!
pub struct RwLock<T> {
    readers: UnsafeCell<AtomicI32>,
    write: UnsafeCell<AtomicI32>,
    value: UnsafeCell<T>,
}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            readers: UnsafeCell::new(AtomicI32::new(0)),
            write: UnsafeCell::new(AtomicI32::new(0)),
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> RGuard<T> {
        RGuard::new(self)
    }

    pub fn write(&self) -> RwGuard<T> {
        RwGuard::new(self)
    }
}

pub struct RGuard<'a, T> {
    rw: &'a RwLock<T>,
}

impl<'a, T> RGuard<'a, T> {
    pub fn new(rw: &'a RwLock<T>) -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            memory_atomic_wait32(rw.write.get() as *mut i32, 1, -1)
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            while (unsafe { &*rw.write.get() }).load(Ordering::SeqCst) == 1 {}
        }
        (unsafe { &*rw.readers.get() }).fetch_add(1, Ordering::SeqCst);
        Self { rw }
    }
}

impl<'a, T> Drop for RGuard<'a, T> {
    fn drop(&mut self) {
        if (unsafe { &*self.rw.readers.get() }).fetch_sub(1, Ordering::SeqCst) == 1 {
            #[cfg(target_arch = "wasm32")]
            unsafe {
                memory_atomic_notify(self.rw.write.get() as *mut i32, 1)
            };
        }
    }
}

impl<'a, T> core::ops::Deref for RGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rw.value.get() }
    }
}

pub struct RwGuard<'a, T> {
    rw: &'a RwLock<T>,
}

impl<'a, T> RwGuard<'a, T> {
    pub fn new(rw: &'a RwLock<T>) -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            memory_atomic_wait32(rw.write.get() as *mut i32, 1, -1);
            (&*rw.write.get()).store(1, Ordering::SeqCst);
            memory_atomic_wait32(rw.readers.get() as *mut i32, 1, -1);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            while (unsafe { &*rw.write.get() }).load(Ordering::SeqCst) == 1 {}
            (unsafe { &*rw.write.get() }).store(1, Ordering::SeqCst);
            while (unsafe { &*rw.readers.get() }).load(Ordering::SeqCst) != 0 {}
        }
        Self { rw }
    }
}

impl<'a, T> Drop for RwGuard<'a, T> {
    fn drop(&mut self) {
        (unsafe { &*self.rw.write.get() }).store(0, Ordering::SeqCst);
        #[cfg(target_arch = "wasm32")]
        unsafe {
            memory_atomic_notify(self.rw.write.get() as *mut i32, 1)
        };
    }
}

impl<'a, T> core::ops::Deref for RwGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rw.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for RwGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.rw.value.get() }
    }
}

unsafe impl<T> Send for RwLock<T> {}
unsafe impl<T> Sync for RwLock<T> {}
unsafe impl<T> Send for Mutex<T> {}
unsafe impl<T> Sync for Mutex<T> {}
