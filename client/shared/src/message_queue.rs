use crate::mem::{atomic_read_u8, MESSAGE_QUEUE, MESSAGE_QUEUE_ELEMENT_COUNT};
use rask_engine::events::{Event, KeyModifier, MouseEvent};

#[repr(C, u32)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Message {
    None,
    KeyDown(KeyModifier, u32) = 1, // 1
    KeyUp(KeyModifier, u32) = 2,
    KeyPress(u32, u16) = 3,
    MouseDown(MouseEvent) = 5, //5
    MouseUp(MouseEvent) = 6,
    RequestAlloc { id: u32, size: u32 } = 7, //7
    ResourcePush(u32) = 8,                   // id
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
impl Outbound {
    pub fn to_js(&self) -> js_sys::Uint32Array {
        let len = std::mem::size_of::<Outbound>() as u32;
        let msg = js_sys::Uint32Array::new_with_length(len);
        let buf: &[u32] = unsafe {
            std::slice::from_raw_parts(self as *const Outbound as *const u32, len as usize)
        };
        for i in 0..len {
            msg.set_index(i, buf[i as usize]);
        }
        msg
    }
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
        let e = std::mem::take(&mut self.payload);
        if self.get_writing() == 0 {
            Some(e)
        } else {
            None
        }
    }
}

#[derive(Default)]
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

    #[allow(clippy::mem_discriminant_non_enum)]
    pub fn pop<T: Sized + Clone + Default + std::fmt::Debug>(&mut self) -> T {
        loop {
            let e = unsafe { self.get_mut(self.reader_index as usize).unwrap() };
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
