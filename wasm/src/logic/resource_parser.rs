use std::collections::HashMap;
use std::convert::TryInto;

use crate::communication::message_queue::Message;
use crate::communication::RESOURCE_TABLE;
use crate::ClientError;
use rask_engine::network::{
    packet::{self, ResourceData},
    protocol::{self, resource_types},
};
use rask_engine::resources::{
    registry::{CharacterInfo, ResourceInfo, ResourceVariant, RESOURCE_COUNT},
    Character, GetStore, Texture,
};
use rask_engine::EngineError;

#[derive(Debug)]
/// Used to handle the resources management with `main.js`.
pub struct ResourceParser {
    buffer_table: HashMap<u32, (*const u8, u32)>,
    char_parts_table: HashMap<u32, [u32; 3]>,
    mapping_table: HashMap<u32, (u32, u32, ResourceVariant)>,
    dyn_resource_id: u32,
}

impl ResourceParser {
    pub fn new() -> Self {
        Self {
            buffer_table: HashMap::new(),
            char_parts_table: HashMap::new(),
            mapping_table: HashMap::new(),
            dyn_resource_id: RESOURCE_COUNT as u32,
        }
    }

    /// Fetch resource via javascript
    pub fn fetch_resource(&mut self, info: ResourceInfo) {
        if let ResourceVariant::Sound = info.variant {
            #[cfg(target_arch = "wasm32")]
            Message::PrepareAudio(info.id, info.path).send();
            return;
        }
        #[cfg(target_arch = "wasm32")]
        Message::FetchResource(info.id, info.path).send();
        self.mapping_table
            .insert(info.id, (info.id, 0, info.variant));
    }

    /// Fetch character resource via javascript
    pub fn fetch_character_resource(&mut self, info: CharacterInfo) {
        #[cfg(target_arch = "wasm32")]
        Message::FetchResource(self.dyn_resource_id, info.texture).send();
        self.mapping_table.insert(
            self.dyn_resource_id,
            (info.id, 0, ResourceVariant::Character),
        );
        #[cfg(target_arch = "wasm32")]
        Message::FetchResource(self.dyn_resource_id + 1, info.animation).send();
        self.mapping_table.insert(
            self.dyn_resource_id + 1,
            (info.id, 1, ResourceVariant::Character),
        );
        #[cfg(target_arch = "wasm32")]
        Message::FetchResource(self.dyn_resource_id + 2, info.atlas).send();
        self.mapping_table.insert(
            self.dyn_resource_id + 2,
            (info.id, 2, ResourceVariant::Character),
        );
        self.char_parts_table.insert(info.id, [0, 0, 0]);
        self.dyn_resource_id += 3;
    }

    /// Fetch resource via javascript
    pub fn fetch_resource(&mut self, info: ResourceInfo) {
        Message::FetchResource(info.id, info.path).send();
        self.mapping_table
            .insert(info.id, (info.id, 0, info.variant));
    }

    /// Fetch character resource via javascript
    pub fn fetch_character_resource(&mut self, info: CharacterInfo) {
        Message::FetchResource(self.dyn_resource_id, info.texture).send();
        self.mapping_table.insert(
            self.dyn_resource_id,
            (info.id, 0, ResourceVariant::Character),
        );
        Message::FetchResource(self.dyn_resource_id + 1, info.animation).send();
        self.mapping_table.insert(
            self.dyn_resource_id + 1,
            (info.id, 1, ResourceVariant::Character),
        );
        Message::FetchResource(self.dyn_resource_id + 2, info.atlas).send();
        self.mapping_table.insert(
            self.dyn_resource_id + 1,
            (info.id, 2, ResourceVariant::Character),
        );
        self.char_parts_table.insert(info.id, [0, 0, 0]);
        self.dyn_resource_id += 3;
    }

    /// Allocates a new buffer and returns the pointer to it.
    pub fn alloc(&mut self, id: u32, size: u32) {
        log::trace!("allocating {} bytes for resource {}", size, id);
        let ptr = self.alloc_buffer(id, size);
        Message::AllocatedBuffer {
            id,
            ptr: ptr as u32,
        }
        .send();
    }

    /// Assumes a resource has been written to the buffer `id` and parses its content.
    pub fn parse(&mut self, id: u32) -> Result<(), ClientError> {
        let mapping = self.mapping_table.get(&id);
        if self.buffer_table.contains_key(&id) {
            if let Some(&mapping) = mapping {
                self.parse_fetched_data(id, mapping)
            } else {
                self.parse_ws_package(id)
            }
        } else {
            Err(ClientError::ResourceError(
                "Requested Buffer not allocated".into(),
            ))
        }
    }

    fn parse_ws_package(&mut self, id: u32) -> Result<(), ClientError> {
        let data = self.get_buffer(id).unwrap();
        let msg = packet::WebSocketPacket::deserialize(&data)?;
        log::trace!("parsing: optcode: {}", msg.op_code);
        assert_eq!(msg.op_code, protocol::op_codes::PUSH_RESOURCE);
        if let packet::PacketVariant::PushResource(data) = msg.payload {
            match data.res_type {
                resource_types::TEXTURE => ResourceParser::parse_texture(data)?,
                resource_types::CHARACTER => ResourceParser::parse_char(data)?,
                _ => {
                    self.dealloc_buffer(id);
                    return Err(ClientError::ResourceError(
                        "unknown ResourceType while parsing".into(),
                    ));
                }
            }
        }
        self.dealloc_buffer(id);
        Ok(())
    }

    fn parse_fetched_data(
        &mut self,
        id: u32,
        mapping: (u32, u32, ResourceVariant),
    ) -> Result<(), ClientError> {
        let (parent_id, part_id, variant) = mapping;
        match variant {
            ResourceVariant::Texture => {
                let data = self.get_buffer(id).ok_or_else(|| {
                    ClientError::ResourceError(format!(
                        "Tried to parse resource id {} for wich no buffer is allocated",
                        id
                    ))
                })?;
                ResourceParser::store_texture(parent_id, data)?
            }
            ResourceVariant::Character => {
                let parts = self.char_parts_table.get_mut(&parent_id).unwrap();
                parts[part_id as usize] = id;
                let parts = *parts;

                if parts.iter().all(|x| *x != 0) {
                    let tex = self.get_buffer(parts[0]).unwrap();
                    let anim = self.get_buffer(parts[1]).unwrap();
                    let atlas = self.get_buffer(parts[2]).unwrap();
                    ResourceParser::parse_char_from_parts(parent_id, tex, anim, atlas)?;
                    for i in parts.iter() {
                        self.dealloc_buffer(*i);
                        self.mapping_table.remove(i);
                    }
                    self.char_parts_table.remove(&parent_id);
                }
            }
            _ => {
                self.dealloc_buffer(id);
                return Err(ClientError::ResourceError(
                    "unknown ResourceType while parsing".into(),
                ));
            }
        }
        Ok(())
    }

    fn parse_texture(res: packet::NetworkResource) -> Result<(), ClientError> {
        match res.data {
            ResourceData::ResourceVec(image) => ResourceParser::store_texture(res.res_id, image),
            _ => {
                Err(EngineError::ResourceType("buffer does not contain texture data".into()).into())
            }
        }
    }

    fn store_texture(id: u32, image: &[u8]) -> Result<(), ClientError> {
        log::info!("decoding texture {} len: {}", id, image.len());
        let img = Texture::from_memory(image)?;
        RESOURCE_TABLE.write().store(img, id as usize)?;
        Ok(())
    }

    fn parse_char(res: packet::NetworkResource) -> Result<(), ClientError> {
        log::info!("decoding char {}", res.res_id);
        let chr: Character = res.data.try_into()?;
        RESOURCE_TABLE
            .write()
            .store(Box::new(chr), res.res_id as usize)?;
        Ok(())
    }

    fn parse_char_from_parts(
        id: u32,
        texture: &[u8],
        animation: &[u8],
        atlas: &[u8],
    ) -> Result<(), ClientError> {
        let chr: Character = Character::from_memory(texture, animation, atlas)?;
        RESOURCE_TABLE.write().store(Box::new(chr), id as usize)?;
        Ok(())
    }

    fn alloc_buffer(&mut self, id: u32, length: u32) -> *const u8 {
        log::trace!("Allocating buffer {} with length of {} bytes", id, length);
        let layout = unsafe { std::alloc::Layout::from_size_align_unchecked(length as usize, 4) };
        let ptr = unsafe { std::alloc::alloc(layout) };
        self.buffer_table.insert(id, (ptr, length));
        ptr
    }

    fn get_buffer(&self, id: u32) -> Option<&[u8]> {
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
}
