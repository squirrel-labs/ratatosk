use super::protocol::{op_codes, resource_types, Opcode};
use crate::error::EngineError;
use crate::resources::registry::{CharacterInfo, ResourceInfo, ResourceVariant};
use std::io::Read;

pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}
pub trait ReadResource {
    fn read_from_file(&self, res_path: &str) -> Option<Vec<u8>>;
}

#[repr(C)]
pub struct WebsocketPacket<'a> {
    op_code: Opcode,
    payload: PacketVariant<'a>,
}

#[repr(C)]
#[derive(Clone, Debug, Copy, Default)]
/// The GameState contains data to be sent over the network and is read by main.js
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    /// Encodes actions the player takes + status effects e.g. poisoned
    pub player_state: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player_x: 0.0,
            player_y: 0.0,
            player_state: 0,
        }
    }
}

#[repr(C)]
pub enum PacketVariant<'a> {
    PushResource(NetworkResource<'a>),
    PushGameState(GameState),
}
#[repr(C)]
pub struct NetworkResource<'a> {
    res_type: u32,
    res_id: u32,
    data: ResourceData<'a>,
}

#[repr(C)]
pub enum ResourceData<'a> {
    ResourceVec(&'a [u8]),
    CharacterVec {
        texture_len: u32,
        atlas_len: u32,
        animation_len: u32,
        data: &'a [u8],
    },
}

impl<'a> Serialize for ResourceData<'a> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        match self {
            Self::ResourceVec(data) => buf.extend(*data),
            Self::CharacterVec {
                texture_len,
                atlas_len,
                animation_len,
                data,
            } => {
                add_u32_to_vec(buf, *texture_len);
                add_u32_to_vec(buf, *atlas_len);
                add_u32_to_vec(buf, *animation_len);
                buf.extend(*data);
            }
        }
    }
}
impl<'a> ResourceData<'a> {
    pub fn deserialize(buf: &'a [u8], res_type: u32) -> Result<Self, EngineError> {
        match res_type {
            resource_types::TEXTURE | resource_types::SOUND => Ok(Self::ResourceVec(buf)),
            resource_types::CHARACTER => Ok(Self::CharacterVec {
                texture_len: u32_from_le(buf)?,
                atlas_len: u32_from_le(&buf[4..])?,
                animation_len: u32_from_le(&buf[8..])?,
                data: buf,
            }),
            _ => Err(EngineError::ResourceType(format!(
                "failed to parse resource type {}",
                res_type
            ))),
        }
    }
}
impl<'a> Serialize for WebsocketPacket<'a> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        add_u32_to_vec(buf, self.op_code);
        self.payload.serialize(buf);
    }
}
impl<'a> WebsocketPacket<'a> {
    pub fn deserialize(buf: &'a [u8]) -> Result<Self, EngineError> {
        let op_code = u32_from_le(buf)?;
        Ok(Self {
            op_code,
            payload: PacketVariant::deserialize(&buf[4..], op_code)?,
        })
    }
}
impl<'a> Serialize for NetworkResource<'a> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        add_u32_to_vec(buf, self.res_type);
        add_u32_to_vec(buf, self.res_id);
        self.data.serialize(buf);
    }
}
impl<'a> NetworkResource<'a> {
    fn deserialize(buf: &'a [u8]) -> Result<Self, EngineError> {
        let res_type = u32_from_le(buf)?;
        Ok(Self {
            res_type,
            res_id: u32_from_le(&buf[4..])?,
            data: ResourceData::deserialize(&buf[8..], res_type)?,
        })
    }
}

impl<'a> Serialize for PacketVariant<'a> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        match self {
            Self::PushResource(data) => data.serialize(buf),
            Self::PushGameState(data) => data.serialize(buf),
        }
    }
}
impl<'a> PacketVariant<'a> {
    fn deserialize(buf: &'a [u8], packet_variant: u32) -> Result<Self, EngineError> {
        match packet_variant {
            op_codes::PUSH_RESOURCE => {
                NetworkResource::deserialize(buf).map(PacketVariant::PushResource)
            }
            op_codes::PUSH_GAMESTATE => {
                GameState::deserialize(buf).map(PacketVariant::PushGameState)
            }
            _ => Err(EngineError::Network(format!(
                "failed to parse websocket optcode {}",
                packet_variant
            ))),
        }
    }
}

impl Serialize for GameState {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        })
    }
}
#[allow(clippy::cast_ptr_alignment)]
impl GameState {
    fn deserialize(buf: &[u8]) -> Result<Self, EngineError> {
        if buf.as_ptr() as usize & (std::mem::align_of::<Self>() - 1) != 0 {
            Err(EngineError::ResourceFormat(
                "Tried to read a game state form an unaligned ".into(),
            ))
        } else {
            Ok(unsafe { *(buf.as_ptr() as *const Self) })
        }
    }
}

pub fn add_u32_to_vec(buf: &mut Vec<u8>, n: u32) {
    buf.extend_from_slice(&n.to_le_bytes())
}

pub fn u32_from_le(barry: &[u8]) -> Result<u32, EngineError> {
    use std::convert::TryInto;
    let arr: [u8; 4] = barry
        .try_into()
        .map_err(|_| EngineError::ResourceFormat("invalid index in character binary".into()))?;
    Ok(u32::from_le_bytes(arr))
}

fn read_to_vec(path: &str, buf: &mut Vec<u8>) -> Result<(), EngineError> {
    let mut file = std::fs::File::open(path)?;
    file.read_to_end(buf)?;
    Ok(())
}

impl ReadResource for ResourceInfo {
    fn read_from_file(&self, res_path: &str) -> Option<Vec<u8>> {
        let mut buf = Vec::new();
        let mut res = Vec::new();
        read_to_vec(format!("{}/{}", res_path, self.path).as_str(), &mut res).ok()?;
        res.push(0x0a);
        WebsocketPacket {
            op_code: op_codes::PUSH_RESOURCE,
            payload: {
                PacketVariant::PushResource(NetworkResource {
                    res_type: self.variant as u32,
                    res_id: self.id,
                    data: ResourceData::ResourceVec(&res),
                })
            },
        }
        .serialize(&mut buf);
        Some(buf)
    }
}
impl ReadResource for CharacterInfo {
    fn read_from_file(&self, res_path: &str) -> Option<Vec<u8>> {
        let mut buf = Vec::new();
        let mut res = Vec::new();
        read_to_vec(format!("{}/{}", res_path, self.texture).as_str(), &mut res).ok()?;
        let texture_len = buf.len() as u32;
        read_to_vec(format!("{}/{}", res_path, self.atlas).as_str(), &mut res).ok()?;
        let atlas_len = buf.len() as u32 - texture_len;
        read_to_vec(
            format!("{}/{}", res_path, self.animation).as_str(),
            &mut res,
        )
        .ok()?;
        buf.push(0x0a);
        let skeleton_len = buf.len() as u32 - (atlas_len + texture_len);

        WebsocketPacket {
            op_code: op_codes::PUSH_RESOURCE,
            payload: {
                PacketVariant::PushResource(NetworkResource {
                    res_type: ResourceVariant::Character as u32,
                    res_id: self.id,
                    data: ResourceData::CharacterVec {
                        texture_len,
                        atlas_len,
                        animation_len: skeleton_len,
                        data: &res,
                    },
                })
            },
        }
        .serialize(&mut buf);
        Some(buf)
    }
}
