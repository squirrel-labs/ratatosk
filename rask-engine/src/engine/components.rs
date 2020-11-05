use crate::io;
use crate::math::Vec2;
use crate::resources::{
    self,
    registry::{CharacterInfo, ResourceInfo},
};
use specs::{prelude::*, Component};

#[derive(Debug, Default)]
pub struct Gravitation(pub Vec2);

#[derive(Debug, Default)]
pub struct TextureIds(pub Vec<u32>);

#[derive(Debug, Default)]
pub struct DeltaTime(pub std::time::Duration);

#[derive(Debug, Default)]
pub struct ElapsedTime(pub std::time::Duration);

pub struct SystemApi(pub(super) Box<dyn io::SystemApi>);

impl Default for SystemApi {
    fn default() -> Self {
        Self(Box::new(io::DummySystemApi))
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
pub struct Animation {
    pub id: u32,
    pub animation: String,
}

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub id: u32,
    pub scale_x: f32,
    pub scale_y: f32,
}

#[derive(Debug, Default, Clone, Copy, Component)]
#[storage(NullStorage)]
pub struct Static;

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
