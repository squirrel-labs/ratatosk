use crate::mem::{atomic_read_u8, atomic_write_u8, MESSAGE_QUEUE, MESSAGE_QUEUE_ELEMENT_COUNT};

#[repr(C, u8)]
#[derive(Debug, Clone)]
pub enum Message {
    None,
    KeyDown(u8, i32),
    KeyUp(u8, i32),
    KeyPress(u16),
    TextInput(bool),
    Click { x: i32, y: i32 },
    ResizeWindow { x: i32, y: i32 },
    ResquestAlloc(i32),
}

impl Default for Message {
    fn default() -> Self {
        Message::None
    }
}

#[repr(C, align(16))]
pub struct MessageQueueElement<T: Sized + Clone> {
    writing: u8,
    payload: T,
}

impl<T: Sized + Clone + Default> MessageQueueElement<T> {
    fn get_writing(&self) -> u8 {
        unsafe { atomic_read_u8(&self.writing) }
    }
    fn read(&mut self) -> Option<T> {
        let e = std::mem::replace(&mut self.payload, T::default());
        if self.get_writing() == 0 {
            Some(e)
        } else {
            None
        }
    }
}

pub struct MessageQueueReader {
    /// the index of the next element to be read
    reader_index: u32,
}

impl MessageQueueReader {
    pub fn length() -> usize {
        MESSAGE_QUEUE_ELEMENT_COUNT
    }

    pub fn new<T: Sized + Clone + Default>() -> Self {
        let mut queue = MessageQueueReader { reader_index: 0 };
        unsafe {
            queue.init::<T>();
        }
        queue
    }

    unsafe fn init<T: Sized + Clone + Default>(&mut self) {
        for i in 0..Self::length() {
            if let Some(e) = self.get_mut::<T>(i) {
                e.payload = T::default();
            }
        }
    }

    unsafe fn get_mut<T: Sized + Clone>(
        &mut self,
        n: usize,
    ) -> Option<&mut MessageQueueElement<T>> {
        core::slice::from_raw_parts_mut(
            MESSAGE_QUEUE as *mut MessageQueueElement<T>,
            Self::length(),
        )
        .get_mut(n)
    }

    pub fn pop<T: Sized + Clone + Default>(&mut self) -> T {
        loop {
            let e = unsafe { self.get_mut(self.reader_index as usize).unwrap() };
            let e = e.read();
            let none = T::default();
            if let Some(none) = e {
                return none;
            }
            self.reader_index += 1;
            if self.reader_index as usize >= Self::length() {
                self.reader_index = 0;
            }
            match e {
                None => continue,
                msg => return msg,
            }
        }
    }
}
