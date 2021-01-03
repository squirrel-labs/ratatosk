use crate::collide::Collidable;
use crate::io;
use crate::math::{Mat3, Vec2};
use crate::resources::{
    self,
    registry::{CharacterInfo, ResourceInfo},
};
use specs::{prelude::*, Component};
use specs_hierarchy::{Hierarchy, Parent as PParent};
use std::collections::HashMap;

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
pub struct Health {
    pub value: u32,
    pub max_value: u32,
}

#[derive(Debug, Clone, Component)]
pub struct Damaging {
    pub damage: f32,
}

#[derive(Debug, Clone, Component)]
pub struct Vulnerable {
    pub armor: f32,
}

#[derive(Debug, Default, Clone, Component)]
#[storage(NullStorage)]
pub struct Terrain;

#[derive(Debug, Clone)]
pub struct SubCollider {
    pub collider: Collidable,
    pub parent: Entity,
}

impl PParent for SubCollider {
    fn parent_entity(&self) -> Entity {
        self.parent
    }
}

impl Component for SubCollider {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Debug, Clone)]
pub enum HitboxType {
    Damaging,
    Vulnerable,
    Repulsion,
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub mapping: HashMap<u32, HitboxType>,
    pub default: HitboxType,
}

impl Component for Collider {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl PParent for Collider {
    fn parent_entity(&self) -> Entity {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub id: u32,
    pub animation: String,
    pub start: f32,
}

impl PParent for Animation {
    fn parent_entity(&self) -> Entity {
        unimplemented!()
    }
}

impl Component for Animation {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Scale(pub Vec2);

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub id: u32,
    pub sub_id: u64,
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub mat3: Mat3,
    pub parent: Entity,
}

impl Component for Transform {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl PParent for Transform {
    fn parent_entity(&self) -> Entity {
        self.parent
    }
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Mass(pub f32);

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
