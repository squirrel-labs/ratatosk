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
        resources! {0, $(($name, $variant, $val)),*}
    };
    ($num:expr, ($name:ident, $variant:ident, $val:expr), $(($name2:ident, $variant2:ident, $val2:expr)),*) => {
        parse_resource! {$num, $name, $variant, $val}
        resources! {$num+1, $(($name2, $variant2, $val2)),*}
    };
    ($num:expr, ($name:ident, $variant:ident, $val:expr)) => {
        parse_resource! {$num, $name, $variant, $val}
    };
}

#[derive(Debug)]
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

resources! {
    (IMAGE1,            Texture,        "see game_context.rs"    ),
    (IMAGE2,            Texture,        "see game_context.rs"    ),
    (UNUSED,            Character,      Character {
                          texture:   "textur.png",
                          atlas:     "atlas.atlas",
                          animation: "animation.anim"
    }                                                            ),
    (USED_TEXTURE_IDS,  TextureIds,     ""                       )
}
