use crate::communication::RESOURCE_TABLE;
use crate::ClientError;
use rask_engine::resources::{registry, GetStore};
use std::collections::HashMap;

/// Used to handle the ressoures management with main.js
pub struct ResourceParser {
    buffer_table: HashMap<u32, (*const u8, u32)>,
}

impl ResourceParser {
    pub fn new() -> Self {
        Self {
            buffer_table: HashMap::new(),
        }
    }
    /// Allocates a new buffer and returns the pointer to it
    pub fn alloc(&mut self, id: u32, size: u32) {
        log::trace!("allocating {} bytes for resource {}", size, id);
        let ptr = self.alloc_buffer(id, size);
        use crate::communication::message_queue::OutboundMessage;
        OutboundMessage::RescourceAlloc {
            id,
            ptr: ptr as u32,
        }
        .send();
    }
    /// Assumes a resource has been written to the buffer `id` and prases its content
    pub fn parse(&mut self, id: u32) -> Result<(), ClientError> {
        if let Some(data) = self.get_buffer(id) {
            const TEXTURE: u32 = registry::ResourceVariant::Texture as u32;
            const CHARACTER: u32 = registry::ResourceVariant::Character as u32;
            match registry::u32_from_le(&data[4..8])? {
                TEXTURE => ResourceParser::parse_texture(id, &data[12..])?,
                CHARACTER => ResourceParser::parse_char(id, &data[12..])?,
                _ => {
                    self.dealloc_buffer(id);
                    return Err(ClientError::ResourceError(
                        "unknown ResourceType while parsing".into(),
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
    fn parse_texture(id: u32, data: &[u8]) -> Result<(), ClientError> {
        log::info!("decoding texture {} len: {}", id, data.len(),);
        let img = rask_engine::resources::Texture::from_png_stream(data)?;
        RESOURCE_TABLE.write().store(img, id as usize)?;
        Ok(())
    }
    fn parse_char(id: u32, data: &[u8]) -> Result<(), ClientError> {
        log::info!("decoding char {} len: {}", id, data.len(),);
        let chr: Result<rask_engine::resources::Character, rask_engine::EngineError> =
            ResourceData::deserialize(
                &data[12..],
                rask_engine::network::protocol::resource_types::CHARACTER,
            )?
            .try_into();
        RESOURCE_TABLE.write().store(Box::new(chr?), id as usize)?;
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
}
