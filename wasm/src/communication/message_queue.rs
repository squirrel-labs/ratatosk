//! The message queue handles communication between the `main.js` and the logic thread.

use std::sync::atomic::AtomicBool;

use rask_engine::events::{Event, KeyModifier, MouseEvent};
use rask_engine::network::protocol::op_codes;

pub const MESSAGE_QUEUE_ELEMENT_COUNT: usize = 128;

#[repr(C, u32)]
#[derive(Debug, Clone)]
#[non_exhaustive]
/// Messages sent by the `main.js`.
pub enum Message {
    None = op_codes::NONE,

    // User interaction handling
    KeyDown(KeyModifier, u32) = op_codes::KEY_DOWN,
    KeyUp(KeyModifier, u32) = op_codes::KEY_UP,
    KeyPress(u32, u16) = op_codes::KEY_PRESS,
    MouseDown(MouseEvent) = op_codes::MOUSE_DOWN,
    MouseUp(MouseEvent) = op_codes::MOUSE_UP,
    /// Ask javascript to set the TextMode on or off.
    TextMode(bool) = op_codes::SET_TEXT_MODE,
    /// Wrapper for game events to be relayed to the server.
    EngineEvent(Event) = op_codes::PUSH_ENGINE_EVENT,

    // Resorce Handling
    RequestAlloc {
        id: u32,
        size: u32,
    } = op_codes::REQUEST_ALLOCATION,
    DoneWritingResource(u32) = op_codes::DONE_WRITING_RESOURCE,
    PushResource(u32) = op_codes::PUSH_RESOURCE,
    /// Rust finshed allocating the requested buffer.
    AllocatedBuffer {
        id: u32,
        ptr: u32,
    } = op_codes::ALLOCATED_BUFFER,
    /// Ask javascript to fetch the requested resource.
    /// In response to this, javascript will fetch the resource and send a RequestAlloc Event.
    /// The rest follows the standard resource flow.
    FetchResource(u32, &'static str) = op_codes::FETCH_RESOURCE,

    // Audio
    /// Ask javascript to fetch the requested sound track.
    PrepareAudio(u32, &'static str) = op_codes::PREPARE_AUDIO,
    AudioLoaded(u32) = op_codes::AUDIO_LOADED,
    PlaySound(u32) = op_codes::PLAY_SOUND,
    StopSound(u32) = op_codes::STOP_SOUND,

    // Misc Management Commands
    /// Send memory offsets to javascript.
    Memory {
        sync_addr: u32,
        queue_addr: u32,
        queue_length: u32,
        element_size: u32,
        game_state_size: u32,
    } = op_codes::MEMORY_OFFSETS,
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

    #[cfg(target_arch = "wasm32")]
    pub fn send(&self) {
        let msg = self.to_slice();
        log::trace!("sending {:?}", self);
        unsafe { post_to_main(msg.as_ptr() as u32, msg.len() as u32) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn send(&self) {
        panic!("Can't send messages to javascript when not running on wasm");
    }
}

extern "C" {
    pub fn post_to_main(ptr: u32, len: u32);
}

#[repr(C, align(32))]
#[derive(Debug)]
/// Wrapper for Message Object.
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
#[repr(C)]
/// Abstracts the communication with the main thread.
pub struct MessageQueue {
    /// The index of the next element to be read.
    data: [MessageQueueElement; MESSAGE_QUEUE_ELEMENT_COUNT],
    reader_index: u32,
}

impl MessageQueue {
    pub fn new() -> Self {
        let bytes = [0u8; std::mem::size_of::<MessageQueueElement>() * MESSAGE_QUEUE_ELEMENT_COUNT];

        MessageQueue {
            reader_index: 0,
            data: unsafe { std::mem::transmute(bytes) },
        }
    }

    pub fn pop(&mut self) -> Message {
        loop {
            let e = &mut self.data[self.reader_index as usize];
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

    /// Push an outbound Message to the main thread.
    pub fn push(&self, msg: Message) {
        msg.send();
    }

    pub fn pos(&self) -> usize {
        self.data.as_ptr() as *const u8 as usize
    }
}
