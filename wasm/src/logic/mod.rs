//! The GameContext contains the logic state and game engine.
//! Its main purpose is to handle events and execute the game engine.

#[cfg(target_arch = "wasm32")]
use crate::mem;
use crate::{
    communication::{
        message_queue::{Message, MessageQueueElement, MessageQueueReader},
        state::State,
        DOUBLE_BUFFER, RESOURCE_TABLE,
    },
    error::ClientError,
};

use rask_engine::events::{Event, Key};
use rask_engine::resources::{registry, GetStore};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
static mut MESSAGES: &mut [MessageQueueElement<Message>] = &mut [MessageQueueElement::new()];
pub struct GameContext {
    state: State,
    tick_nr: u64,
    message_queue: MessageQueueReader<'static, Message>,
    buffer_table: HashMap<u32, (*const u8, u32)>,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        #[cfg(target_arch = "wasm32")]
        let message_queue = unsafe {
            MessageQueueReader::from_memory(
                *mem::MESSAGE_QUEUE as *mut MessageQueueElement<Message>,
                mem::MESSAGE_QUEUE_ELEMENT_COUNT as usize,
            )
        };
        #[cfg(not(target_arch = "wasm32"))]
        let message_queue = MessageQueueReader::new(unsafe { MESSAGES });
        Ok(Self {
            state: State::default(),
            tick_nr: 0,
            message_queue,
            buffer_table: HashMap::new(),
        })
    }

    fn push_state(&mut self) {
        let mut writer = DOUBLE_BUFFER.lock();
        *writer = self.state;
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        loop {
            let msg = self.message_queue.pop();
            if let Message::None = msg {
                break;
            }
            log::info!("{:?}", msg);
            self.handle_message(msg)?;
        }

        self.push_state();
        self.tick_nr += 1;
        Ok(())
    }

    fn alloc_buffer(&mut self, id: u32, length: u32) -> *const u8 {
        let layout = unsafe { std::alloc::Layout::from_size_align_unchecked(length as usize, 4) };
        let ptr = unsafe { std::alloc::alloc(layout) };
        self.buffer_table.insert(id, (ptr, length));
        ptr
    }
    fn get_buffer(&mut self, id: u32) -> Option<&[u8]> {
        if let Some((ptr, length)) = self.buffer_table.get(&id) {
            unsafe { Some(std::slice::from_raw_parts(*ptr, *length as usize)) }
        } else {
            None
        }
    }
    fn dealloc_buffer(&mut self, id: u32) {
        if let Some((ptr, length)) = self.buffer_table.remove(&id) {
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(length as usize, 4);
                std::alloc::dealloc(ptr as *mut u8, layout)
            }
        }
    }
    fn handle_message(&mut self, message: Message) -> Result<Option<Event>, ClientError> {
        match message {
            Message::KeyDown(modifier, hash) => Ok(Some(Event::KeyDown(modifier, Key::from(hash)))),
            Message::KeyUp(modifier, hash) => Ok(Some(Event::KeyUp(modifier, Key::from(hash)))),
            Message::MouseDown(event) => Ok(Some(Event::MouseDown(event))),
            Message::MouseUp(event) => Ok(Some(Event::MouseUp(event))),
            Message::KeyPress(t, code) => Ok(Some(Event::KeyPress(t as u16, code))),
            Message::RequestAlloc { id, size } => {
                log::trace!("allocating {} bytes for resource {}", size, id);
                let ptr = self.alloc_buffer(id, size);
                use crate::communication::message_queue::Outbound;
                Outbound::RescourceAlloc {
                    id,
                    ptr: ptr as u32,
                }
                .send();
                Ok(None)
            }
            Message::ResourcePush(id) => self.parse_resource(id).map(|_| None),
            _ => Err(ClientError::EngineError("Unknown Message Type".into())),
        }
    }
    fn parse_resource(&mut self, id: u32) -> Result<(), ClientError> {
        if let Some(data) = self.get_buffer(id) {
            const TEXTURE: u32 = registry::ResourceVariant::Texture as u32;
            const CHARACTER: u32 = registry::ResourceVariant::Character as u32;
            match registry::u32_from_le(&data[4..8])? {
                TEXTURE => {
                    log::info!("decoding texture {} len: {}", id, data[12..].len(),);
                    let img = rask_engine::resources::Texture::from_png_stream(&data[12..])?;
                    RESOURCE_TABLE.write().store(img, id as usize)?;
                }
                CHARACTER => {
                    let chr = rask_engine::resources::Character::from_u8(&data[12..])?;
                    RESOURCE_TABLE.write().store(Box::new(chr), id as usize)?;
                }
                _ => {
                    self.dealloc_buffer(id);
                    return Err(ClientError::ResourceError(
                        "unknown RescorceType while parsing".into(),
                    ));
                }
            }
            self.dealloc_buffer(id);
            Ok(())
        } else {
            Err(ClientError::ResourceError(
                "Requested Buffer not allocated".into(),
            ))
        }
    }
}
