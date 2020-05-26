use std::collections::HashMap;
use std::convert::TryInto;

use crate::communication::message_queue::Message;
use crate::communication::RESOURCE_TABLE;
use crate::ClientError;
use rask_engine::network::{
    packet::{self, ResourceData},
    protocol::{self, resource_types},
};
use rask_engine::resources::{Character, GetStore, Texture};
use rask_engine::EngineError;

#[derive(Debug)]
/// Used to handle the resources management with `main.js`.
pub struct ResourceParser {
    buffer_table: HashMap<u32, (*const u8, u32)>,
}

impl ResourceParser {
    pub fn new() -> Self {
        Self {
            buffer_table: HashMap::new(),
        }
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
        if let Some(data) = self.get_buffer(id) {
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
            } else {
            }
            self.dealloc_buffer(id);
            Ok(())
        } else {
            Err(ClientError::ResourceError(
                "Requested Buffer not allocated".into(),
            ))
        }
    }

    fn parse_texture(res: packet::NetworkResource) -> Result<(), ClientError> {
        match res.data {
            ResourceData::ResourceVec(image) => {
                log::info!("decoding texture {} len: {}", res.res_id, image.len());
                let img = Texture::from_png_stream(image)?;
                RESOURCE_TABLE.write().store(img, res.res_id as usize)?;
                Ok(())
            }
            _ => {
                Err(EngineError::ResourceType("buffer does not contain texture data".into()).into())
            }
        }
    }

    fn parse_char(res: packet::NetworkResource) -> Result<(), ClientError> {
        log::info!("decoding char {}", res.res_id);
        let chr: Character = res.data.try_into()?;
        RESOURCE_TABLE
            .write()
            .store(Box::new(chr), res.res_id as usize)?;
        Ok(())
    }

    fn alloc_buffer(&mut self, id: u32, length: u32) -> *const u8 {
        log::trace!("Allocating buffer {} with length of {} bytes", id, length);
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
