use crate::events::Event;
use crate::resources::ResourceTable;
use crate::EngineError;
use crate::math::Vec2;
use specs::prelude::*;

pub struct Level {
}

/// An interface for the game server to interact with the game.
pub trait GameEngine {
    /// Create a new game.
    fn new() -> Self;

    fn load_level(level: Level);

    /// Let the game engine handle the given event.
    /// May cause an `EngineError`.
    fn handle_event(&mut self, event: Event) -> Result<(), EngineError>;

    /// Do a logic tick.
    /// May cause an `EngineError`.
    fn tick(&mut self, dt: i32, res: &ResourceTable) -> Result<(), EngineError>;
}

/// The rask specific implementation of the `GameEngine`
#[allow(dead_code)]
pub struct RaskEngine {
    world: World,
}

// A component contains data
// which is associated with an entity.
#[derive(Debug, Clone, Copy)]
struct Vel(Vec2);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone, Copy)]

struct Pos(Vec2);
impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct Hitbox()

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple component storages,
        // so we get access to all entities which have
        // both a position and a velocity.
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }
}

impl GameEngine for RaskEngine {
    fn new() -> Self {
        Self {
            world: specs::WorldExt::new(),
        }
    }

    fn load_level(level: Level) {}

    fn handle_event(&mut self, event: Event) -> Result<(), EngineError> {
        Ok(())
    }

    fn tick(&mut self, dt: i32, res: &ResourceTable) -> Result<(), EngineError> {
        Ok(())
    }
}
