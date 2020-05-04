use crate::error::EngineError;
use std::io::Read;

macro_rules! parse_resource {
    ($num:expr, $name:ident, Character, $val: tt) => {
        pub const $name: CharacterInfo = CharacterInfo {
            texture: $val.texture,
            atlas: $val.atlas,
            animation: $val.animation,
            id: $num,
        };
    };
    ($num:expr, $name:ident, $variant:ident, $val: expr) => {
        pub const $name: ResourceInfo = ResourceInfo {
            variant: ResourceVariant::$variant,
            path: $val,
            id: $num,
        };
    };
}

macro_rules! resources {
    ($(($name:ident, $variant:ident, $val:expr)),*) => {
        resources! { 0, $(($name, $variant, $val)),* }
    };
    ($num:expr, ($name:ident, $variant:ident, $val:expr), $(($name2:ident, $variant2:ident, $val2:expr)),*) => {
        parse_resource! { $num, $name, $variant, $val }
        resources! { $num+1, $(($name2, $variant2, $val2)),* }
    };
    ($num:expr, ($name:ident, $variant:ident, $val:expr)) => {
        parse_resource! { $num, $name, $variant, $val }
    };
}

pub trait Serialize {
    fn serialize(&self, res_path: &str) -> Option<Vec<u8>>;
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceVariant {
    Texture = 2,
    Character = 3,
    Sound = 4,
    TextureIds,
}

#[derive(Debug)]
pub struct ResourceInfo {
    pub variant: ResourceVariant,
    pub path: &'static str,
    pub id: u32,
}

struct Character {
    pub texture: &'static str,
    pub atlas: &'static str,
    pub animation: &'static str,
}

#[derive(Debug)]
pub struct CharacterInfo {
    pub texture: &'static str,
    pub atlas: &'static str,
    pub animation: &'static str,
    pub id: u32,
}

pub fn add_u32_to_vec(buf: &mut Vec<u8>, n: u32) {
    buf.extend_from_slice(&n.to_le_bytes())
}

pub fn u32_from_le(barry: &[u8]) -> Result<u32, EngineError> {
    use std::convert::TryInto;
    let arr: [u8; 4] = barry
        .try_into()
        .map_err(|_| EngineError::ResourceFormat("invalid index in charakter binary".into()))?;
    Ok(u32::from_le_bytes(arr))
}

fn read_to_vec(path: &str, buf: &mut Vec<u8>) -> Result<(), EngineError> {
    let mut file = std::fs::File::open(path)?;
    file.read_to_end(buf)?;
    Ok(())
}
impl Serialize for ResourceInfo {
    fn serialize(&self, res_path: &str) -> Option<Vec<u8>> {
        let mut buf = Vec::new();
        add_u32_to_vec(&mut buf, 10); // TODO Replace magic
        add_u32_to_vec(&mut buf, self.variant as u32);
        add_u32_to_vec(&mut buf, self.id);
        read_to_vec(format!("{}/{}", res_path, self.path).as_str(), &mut buf).ok()?;
        buf.push(0x0a);
        Some(buf)
    }
}
impl Serialize for CharacterInfo {
    fn serialize(&self, res_path: &str) -> Option<Vec<u8>> {
        let mut buf = Vec::new();

        add_u32_to_vec(&mut buf, 10);
        add_u32_to_vec(&mut buf, ResourceVariant::Character as u32);
        add_u32_to_vec(&mut buf, self.id);

        let mut res = Vec::new();
        read_to_vec(format!("{}/{}", res_path, self.texture).as_str(), &mut buf).ok()?;
        let tex_len = res.len();
        read_to_vec(format!("{}/{}", res_path, self.atlas).as_str(), &mut buf).ok()?;
        let atlas_len = res.len() - tex_len;
        read_to_vec(
            format!("{}/{}", res_path, self.animation).as_str(),
            &mut buf,
        )
        .ok()?;
        let skeleton_len = res.len() - (atlas_len + tex_len);

        add_u32_to_vec(&mut buf, tex_len as u32);
        add_u32_to_vec(&mut buf, atlas_len as u32);
        add_u32_to_vec(&mut buf, skeleton_len as u32);

        buf.push(0x0a);
        buf.append(&mut res);
        Some(buf)
    }
}

resources! {
    (USED_TEXTURE_IDS,  TextureIds,     ""                       ),
    (EMPTY,             Texture,        "empty.png"              ),
    (THIEF,             Texture,        "thief.png"              ),
    (UNUSED,            Character,      Character {
                          texture:   "BoneTest/BoneTest.png",
                          atlas:     "BoneTest/BoneTest.atlas",
                          animation: "BoneTest/BoneTest.json"
    }                                                            )
}
