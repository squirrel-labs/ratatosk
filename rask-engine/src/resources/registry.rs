use crate::network::protocol::resource_types;
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

pub const RESOURCE_COUNT: u32 = 4;

resources! {
    (EMPTY,             Texture,        "empty.png"              ),
    (THIEF,             Texture,        "thief.png"              ),
    (UNUSED,            Character,      Character {
                          texture:   "BoneTest/BoneTest.png",
                          atlas:     "BoneTest/BoneTest.atlas",
                          animation: "BoneTest/BoneTest.json"
    }                                                            )
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ResourceVariant {
    Texture = resource_types::TEXTURE,
    Character = resource_types::CHARACTER,
    Sound = resource_types::SOUND,
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
