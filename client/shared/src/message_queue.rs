use crate::mem::{atomic_read_u8, atomic_write_u8, MESSAGE_QUEUE_ELEMENT_COUNT};

#[repr(C, u16)]
#[derive(Debug, Clone)]
pub enum Message {
    Unknown,
    KeyDown,
    KeyUp,
    TextInput,
    Click { x: i32, y: i32 },
    ResizeWindow,
    ResquestAlloc,
}

#[repr(align(4))]
pub struct MessageQueueElement<T: Sized + Clone> {
    reading: u8,
    writing: u8,
    payload: T,
}

impl<T: Sized + Clone> MessageQueueElement<T> {
    fn set_reading(&mut self, val: u8) {
        unsafe { atomic_write_u8(&mut self.reading, val) }
    }

    fn get_writing(&self) -> u8 {
        unsafe { atomic_read_u8(&self.writing) }
    }
    fn read(&mut self) -> Option<T> {
        self.set_reading(1);
        if self.get_writing() == 0 {
            let e = self.payload.clone();
            self.set_reading(0);
            Some(e)
        } else {
            None
        }
    }
}

#[repr(align(4))]
pub struct MessageQueue<T: Sized + Clone> {
    /// the index of the next element to be written
    writer_index: u32,
    /// the index of the next element to be read
    reader_index: u32,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: Sized + Clone> MessageQueue<T> {
    pub fn length() -> usize {
        MESSAGE_QUEUE_ELEMENT_COUNT
    }

    fn mem_size() -> usize {
        core::mem::size_of::<T>() * Self::length() + core::mem::size_of::<Self>()
    }

    unsafe fn get_mut(&mut self, n: usize) -> Option<&mut MessageQueueElement<T>> {
        core::slice::from_raw_parts_mut(
            (self as *mut Self as *mut u8).offset(core::mem::size_of::<Self>() as isize)
                as *mut MessageQueueElement<T>,
            Self::length(),
        )
        .get_mut(n)
    }

    pub fn pop(&mut self) -> Option<T> {
        let e = unsafe { self.get_mut(self.reader_index as usize)? };
        let e = e.read()?;
        self.reader_index += 1;
        if self.reader_index as usize >= Self::length() {
            self.reader_index = 0;
        }
        Some(e)
    }
}
