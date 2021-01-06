use crate::io;
use crate::math::Vec2;
use crate::resources::{
    self,
    registry::{CharacterInfo, ResourceInfo},
};
use specs::{prelude::*, Component};
use specs_hierarchy::Parent as PParent;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Gravitation(pub Vec2);

#[derive(Debug, Default)]
pub struct TextureIds(pub HashSet<(u32, u64)>);

#[derive(Debug, Default)]
pub struct DeltaTime(pub std::time::Duration);

#[derive(Debug, Default)]
pub struct ElapsedTime(pub std::time::Duration);

#[derive(Debug, Default)]
pub struct RenderBufferDimensions(pub (u32, u32));

pub struct SystemApi(pub(super) Box<dyn io::SystemApi>);

impl Default for SystemApi {
    fn default() -> Self {
        Self(Box::new(io::DummySystemApi))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Parent {
    pub entity: Entity,
}

impl Component for Parent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl PParent for Parent {
    fn parent_entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
pub struct Vel(pub Vec2);

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
pub struct Pos(pub Vec2);

#[derive(Debug, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Speed(pub f32);

#[derive(Debug, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Animation {
    pub id: u32,
    pub animation: String,
    pub start: f32,
}

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Scale(pub Vec2);

#[derive(Debug, Clone)]
pub struct TextBox {
    pub font: ResourceInfo,
    pub content: String,
    pub fontsize: f32,
    pub color: u32,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl Component for TextBox {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Default for TextBox {
    fn default() -> Self {
        Self {
            font: crate::resources::registry::PIXELFONT,
            content: String::new(),
            fontsize: 40.0,
            color: core::u32::MAX,
            width: None,
            height: None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
#[storage(NullStorage)]
pub struct Glyph;

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub id: u32,
    pub sub_id: u64,
}

#[derive(Debug, Default, Clone, Copy, Component)]
#[storage(NullStorage)]
pub struct Static;

#[derive(Debug, Default, Clone, Copy, Component)]
#[storage(NullStorage)]
pub struct Present;

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
pub struct Resource(pub ResourceInfo);

impl Default for Resource {
    fn default() -> Self {
        Resource(resources::registry::EMPTY)
    }
}

#[derive(Debug, Clone, Copy, Component)]
#[storage(DenseVecStorage)]
pub struct CharacterResource(pub CharacterInfo);
impl Default for CharacterResource {
    fn default() -> Self {
        CharacterResource(resources::registry::CHAR)
    }
}
