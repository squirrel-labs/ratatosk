//! The GameContext contains the logic state and gameengine
//! Its main purpose is to handle events and execute the game engine
use crate::{
    error::ClientError,
    mem::{self, RESOURCE_TABLE_ELEMENT_COUNT},
    message_queue::{Message, MessageQueueReader},
    state::State,
    DOUBLE_BUFFER,
};
use rask_engine::events::{Event, Key};
use rask_engine::network::packet::{u32_from_le, ResourceData};
use rask_engine::resources::{registry, GetStore, ResourceTable, TextureIds};
use std::collections::HashMap;
use std::convert::TryInto;

pub struct GameContext {
    state: State,
    tick_nr: u64,
    resource_table: ResourceTable,
    message_queue: MessageQueueReader,
    buffer_table: HashMap<u32, (*const u8, u32)>,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        let resource_table = unsafe {
            let mut resource_table = ResourceTable::from_memory(
                *mem::RESOURCE_TABLE,
                RESOURCE_TABLE_ELEMENT_COUNT as usize,
            );
            resource_table.clear();
            resource_table.store(
                // TODO move to mutex
                TextureIds {
                    reset_notify: 1,
                    ids: vec![],
                },
                registry::USED_TEXTURE_IDS.id as usize,
            )?;
            resource_table
        };
        log::debug!(
            "resource_table: {}",
            &resource_table as *const ResourceTable as u32
        );
        Ok(Self {
            state: State::default(),
            tick_nr: 0,
            resource_table,
            message_queue: MessageQueueReader::new(),
            buffer_table: HashMap::new(),
        })
    }

    fn push_state(&mut self) {
        let mut writer = DOUBLE_BUFFER.lock();
        *writer = self.state;
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        loop {
            let msg = self.message_queue.pop::<Message>();
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
        let layout = unsafe { std::alloc::Layout::from_size_align_unchecked(length as usize, 1) };
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
                let layout = std::alloc::Layout::from_size_align_unchecked(length as usize, 1);
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
                use crate::message_queue::Outbound;
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
            match u32_from_le(&data[4..8])? {
                TEXTURE => {
                    log::info!("decoding texture {} len: {}", id, data[12..].len(),);
                    let img = rask_engine::resources::Texture::from_png_stream(&data[12..])?;
                    unsafe { self.resource_table.store(img, id as usize) }?;
                }
                CHARACTER => {
                    let chr: Result<rask_engine::resources::Character, rask_engine::EngineError> =
                        ResourceData::deserialize(
                            &data[12..],
                            rask_engine::network::protocol::resource_types::CHARACTER,
                        )?
                        .try_into();
                    unsafe { self.resource_table.store(Box::new(chr?), id as usize) }?;
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