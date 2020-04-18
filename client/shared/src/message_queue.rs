use crate::mem::{atomic_read_u8, MESSAGE_QUEUE, MESSAGE_QUEUE_ELEMENT_COUNT};
use rask_engine::events::{Event, KeyModifier, MouseEvent};

#[repr(C, u32)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Message {
    None,
    KeyDown(KeyModifier, u32), // 1
    KeyUp(KeyModifier, u32),
    KeyPress(u32, u16),
    MouseDown(MouseEvent) = 5, //5
    MouseUp(MouseEvent),
    ResquestAlloc { id: u32, size: u32 }, //7
    ResourcePush(u32),                    // id
}

impl Default for Message {
    fn default() -> Self {
        Message::None
    }
}

#[repr(C, u32)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Outbound {
    RescourceAlloc { id: u32, ptr: u32 } = 0, // The event ids from 0 to 128 are reserved for server to client communication
    Textmode(bool),
    EngineEvent(Event) = 129, // Mark the Message as outbound
}

#[repr(C, align(32))]
#[derive(Debug)]
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

    pub fn new() -> Self {
        MessageQueueReader { reader_index: 0 }
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

    pub fn pop<T: Sized + Clone + Default + std::fmt::Debug>(&mut self) -> T {
        loop {
            let e = unsafe { self.get_mut(self.reader_index as usize).unwrap() };
            //log::info!("bytes are  {:?}", unsafe {std::slice::from_raw_parts_mut(e as *mut MessageQueueElement<_> as *mut u8, 16)});
            let e = e.read();
            if let Some(n) = e.clone() {
                if std::mem::discriminant(&T::default()) == std::mem::discriminant(&n) {
                    return n;
                }
            }
            self.reader_index += 1;
            if self.reader_index as usize >= Self::length() {
                self.reader_index = 0;
            }
            match e {
                None => continue,
                Some(msg) => return msg,
            }
        }
    }
}
