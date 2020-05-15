//! The message queue handels communication between the main.js and the logic thread

use rask_engine::events::{Event, KeyModifier, MouseEvent};
use rask_engine::network::protocol::op_codes;
use std::sync::atomic::AtomicBool;

#[repr(C, u32)]
#[derive(Debug, Clone)]
#[non_exhaustive]
/// Messeges sent by the main.js
pub enum Message {
    None = op_codes::NONE,
    KeyDown(KeyModifier, u32) = op_codes::KEY_DOWN, // 1
    KeyUp(KeyModifier, u32) = op_codes::KEY_UP,
    KeyPress(u32, u16) = op_codes::KEY_PRESS,
    MouseDown(MouseEvent) = op_codes::MOUSE_DOWN, //5
    MouseUp(MouseEvent) = op_codes::MOUSE_UP,
    RequestAlloc { id: u32, size: u32 } = op_codes::REQUEST_ALLOCATION, //7
    DoneWritingResource(u32) = op_codes::DONE_WRITING_RESOURE,          // id
    ResourcePush(u32) = op_codes::PUSH_RESOURCE,                        // id
    AllocatedBuffer { id: u32, ptr: u32 } = op_codes::ALLOCATED_BUFFER, // The event ids from 0 to 128 are reserved for server to client communication
    Memory(u32, u32, u32) = op_codes::MEMORY_OFFSETS,
    Textmode(bool) = op_codes::SET_TEXTMODE,
    EngineEvent(Event) = op_codes::PUSH_ENGINE_EVENT, // Mark the Message as outbound
}

impl Default for Message {
    fn default() -> Self {
        Message::None
    }
}

impl Message {
    pub fn to_slice(&self) -> &[u32] {
        let len = std::mem::size_of::<Message>() as u32;
        unsafe { std::slice::from_raw_parts(self as *const Message as *const u32, len as usize) }
    }
    pub fn send(&self) {
        let msg = self.to_slice();
        log::trace!("sending {:?}", self);
        unsafe { post_to_main(msg.as_ptr() as u32, msg.len() as u32) }
    }
}

extern "C" {
    pub fn post_to_main(ptr: u32, len: u32);
}

#[repr(C, align(32))]
#[derive(Debug)]
/// Wrapper for Message Object
pub struct MessageQueueElement {
    writing: AtomicBool,
    payload: Message,
}
impl From<Message> for MessageQueueElement {
    fn from(message: Message) -> Self {
        Self {
            writing: AtomicBool::new(false),
            payload: message,
        }
    }
}

impl MessageQueueElement {
    fn read(&mut self) -> Option<Message> {
        let e = std::mem::take(&mut self.payload);
        if !*self.writing.get_mut() {
            Some(e)
        } else {
            None
        }
    }
}
impl MessageQueueElement {
    pub const fn new() -> Self {
        Self {
            writing: AtomicBool::new(false),
            payload: Message::None,
        }
    }
}

#[derive(Debug)]
/// Abstracts the communication with the main thread
pub struct MessageQueue<'a> {
    /// the index of the next element to be read
    reader_index: u32,
    data: &'a mut [MessageQueueElement],
}

impl<'a> MessageQueue<'a> {
    /// # Safety
    /// the memory provided to the function has to be valid and must contain valid messages
    pub unsafe fn from_memory(ptr: *mut MessageQueueElement, len: usize) -> Self {
        MessageQueue {
            reader_index: 0,
            data: core::slice::from_raw_parts_mut(ptr, len),
        }
    }

    // add method to create message_queue with a memory location to make testing easier
    pub fn new(data: &'a mut [MessageQueueElement]) -> Self {
        MessageQueue {
            reader_index: 0,
            data,
        }
    }

    fn get_mut(&mut self, n: usize) -> Option<&mut MessageQueueElement> {
        self.data.get_mut(n)
    }

    pub fn pop(&mut self) -> Message {
        loop {
            let e = self
                .get_mut(self.reader_index as usize)
                .expect("Failed to Read MessageQueue");
            let e = e.read();
            if let Some(Message::None) = e {
                return Message::None;
            }
            self.reader_index += 1;
            if self.reader_index as usize >= self.data.len() {
                self.reader_index = 0;
            }
            match e {
                None => continue,
                Some(msg) => return msg,
            }
        }
    }
    /// Push an outbound Message to the main Thread
    pub fn push(&self, msg: Message) {
        msg.send();
    }
}
