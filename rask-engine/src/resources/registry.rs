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
        pub const RESOURCE_COUNT: u32 = $num + 1;
    };
}

resources! {
    (EMPTY,             Texture,        "empty.png"              ),
    (THIEF,             Texture,        "thief.png"              ),
    (CHAR,              Character,      Character {
                          texture:   "Shmief/Shmief.png",
                          atlas:     "Shmief/Shmief.atlas",
                          animation: "Shmief/Shmief.json"
    }                                                            )
}

trait ResourceId {
    fn get_id(&self) -> u32;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ResourceVariant {
    Texture = resource_types::TEXTURE,
    Character = resource_types::CHARACTER,
    Sound = resource_types::SOUND,
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceInfo {
    pub variant: ResourceVariant,
    pub path: &'static str,
    pub id: u32,
}

impl ResourceId for ResourceInfo {
    fn get_id(&self) -> u32 {
        self.id
    }
}

struct Character {
    pub texture: &'static str,
    pub atlas: &'static str,
    pub animation: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct CharacterInfo {
    pub texture: &'static str,
    pub atlas: &'static str,
    pub animation: &'static str,
    pub id: u32,
}

impl ResourceId for CharacterInfo {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl std::convert::From<ResourceInfo> for usize {
    fn from(a: ResourceInfo) -> Self {
        a.get_id() as usize
    }
}
impl std::convert::From<CharacterInfo> for usize {
    fn from(a: CharacterInfo) -> Self {
        a.get_id() as usize
    }
}
