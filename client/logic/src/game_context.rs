use rask_engine::events::{Event, Key};
use rask_engine::resources::{registry, GetStore, ResourceTable, Texture, TextureIds};
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::mem::{RESOURCE_TABLE, RESOURCE_TABLE_ELEMENT_COUNT};
use rask_wasm_shared::sprite::*;
use rask_wasm_shared::{
    message_queue::{Message, MessageQueueReader},
    state::State,
};
use std::collections::HashMap;

pub struct GameContext {
    state: State,
    tick_nr: u64,
    #[allow(dead_code)]
    resource_table: ResourceTable,
    message_queue: MessageQueueReader,
    worker_scope: web_sys::DedicatedWorkerGlobalScope,
    buffer_table: HashMap<u32, (*const u8, u32)>,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        let resource_table = unsafe {
            let mut resource_table =
                ResourceTable::from_memory(RESOURCE_TABLE, RESOURCE_TABLE_ELEMENT_COUNT);
            resource_table.clear();
            resource_table.store(
                TextureIds {
                    reset_notify: 1,
                    ids: vec![],
                },
                registry::USED_TEXTURE_IDS.id as usize,
            )?;
            resource_table
        };
        use wasm_bindgen::JsCast;
        let worker_scope = js_sys::global()
            .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
            .unwrap();

        Ok(Self {
            state: State::default(),
            tick_nr: 0,
            resource_table,
            message_queue: MessageQueueReader::new(),
            worker_scope,
            buffer_table: HashMap::new(),
        })
    }

    fn push_state(&mut self) -> Result<(), ClientError> {
        let mut writer = get_double_buffer().borrow_writer();
        writer.set(self.state);
        Ok(())
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

        if self.state.sprites().len() == 2 {
            self.state.sprites_mut()[1].transform =
                rask_engine::math::Mat3::rotation(0.02) * self.state.sprites_mut()[1].transform;
        }
        self.push_state()?;
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
                log::info!("allocating {} bytes for resource {}", size, id);
                let ptr = self.alloc_buffer(id, size);
                use rask_wasm_shared::message_queue::Outbound;
                let msg = Outbound::RescourceAlloc {
                    id,
                    ptr: ptr as u32,
                }
                .to_js();
                self.worker_scope.post_message(&msg.buffer()).unwrap();
                Ok(None)
            }
            Message::ResourcePush(id) => {
                if let Some(data) = self.get_buffer(id) {
                    const TEXTURE: u32 = registry::ResourceVariant::Texture as u32;
                    const CHARACTER: u32 = registry::ResourceVariant::Character as u32;
                    match registry::u32_from_le(&data[4..8])? {
                        TEXTURE => {
                            log::info!("decoding texture {} len: {}", id, data[12..].len(),);
                            let img =
                                rask_engine::resources::Texture::from_png_stream(&data[12..])?;
                            unsafe { self.resource_table.store(img, id as usize) }?;
                            let mut sprite = Sprite::default();
                            sprite.tex_id = id;
                            self.state.append_sprite(&sprite);
                            if self.state.sprites().len() == 2 {
                                unsafe {
                                    self.resource_table.store(
                                        TextureIds {
                                            reset_notify: 1,
                                            ids: vec![registry::EMPTY.id, registry::THIEF.id],
                                        },
                                        registry::USED_TEXTURE_IDS.id as usize,
                                    )?;
                                }
                            }
                        }
                        CHARACTER => {
                            let chr = rask_engine::resources::Character::from_u8(&data[12..])?;
                            unsafe { self.resource_table.store(Box::new(chr), id as usize) }?;
                        }
                        _ => {
                            self.dealloc_buffer(id);
                            return Err(ClientError::ResourceError(
                                "unknown RescorceType while parsing".into(),
                            ));
                        }
                    }
                    self.dealloc_buffer(id);
                    Ok(None)
                } else {
                    Err(ClientError::ResourceError(
                        "Requested Buffer not allocated".into(),
                    ))
                }
            }
            _ => Err(ClientError::EngineError("Unknown Message Type".into())),
        }
    }
}
