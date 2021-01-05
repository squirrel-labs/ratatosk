use crate::io;
use crate::math::Vec2;
use crate::resources::registry;
use crate::EngineError;
use core::time::Duration;
use specs::prelude::*;
use specs::WorldExt;

mod components;
mod systems;
use components::*;
use systems::*;

const GRAVITY: Vec2 = Vec2::new(0.0, -9.807);

pub struct Level {}

/// An interface for the game server to interact with the game.
pub trait GameEngine {
    /// Create a new game.
    fn new(pool: std::sync::Arc<rayon::ThreadPool>, system: Box<dyn io::SystemApi>) -> Self;

    fn load_level(level: Level);

    /// Do a logic tick.
    /// May cause an `EngineError`.
    fn tick(&mut self, dt: Duration, elapsed: Duration) -> Result<(), EngineError>;
}

/// The rask specific implementation of the `GameEngine`
#[allow(dead_code)]
pub struct RaskEngine {
    world: World,
    tick_dispatcher: Dispatcher<'static, 'static>,
}

impl GameEngine for RaskEngine {
    fn new(pool: std::sync::Arc<rayon::ThreadPool>, mut system: Box<dyn io::SystemApi>) -> Self {
        system.fetch_resource(registry::EMPTY).unwrap();
        system.fetch_resource(registry::SOUND).unwrap();
        system.fetch_resource(registry::PIXELFONT).unwrap();
        system.fetch_character_resource(registry::CHAR).unwrap();

        let mut world: specs::World = specs::WorldExt::new();
        world.insert(Gravitation(GRAVITY));
        world.insert(DeltaTime(Duration::from_millis(10)));
        world.insert(ElapsedTime(Duration::from_millis(0)));
        world.insert(TextureIds(Vec::new()));
        world.insert(SystemApi(system));

        let mut tick_dispatcher = DispatcherBuilder::new()
            .with_pool(pool)
            .with(EventSystem, "events", &[])
            .with(CheckPresentSystem, "check_present", &[]) // does not depend on anything, because resource parsing is handled asynchronously
            .with(UpdateAnimationSystem, "update_anim", &["check_present"])
            .with(MovementSystem, "movement", &["events"])
            .with(GravitationSystem, "gravitation", &["movement"])
            .with(VelocitySystem, "velocity", &["gravitation"])
            .with_thread_local(RenderSystem)
            .build();

        tick_dispatcher.setup(&mut world);
        let _backround = world
            .create_entity()
            .with(Pos(Vec2::new(0.0, 0.0)))
            .with(Sprite {
                id: registry::EMPTY.id,
            })
            .with(Scale(Vec2::new(1.0, 1.0)))
            .with(Static)
            .build();
        let _char = world
            .create_entity()
            .with(Pos(Vec2::new(0.0, -0.8)))
            .with(Vel(Vec2::new(0.0, 0.0)))
            .with(Speed(0.2))
            .with(Animation {
                id: registry::CHAR.id,
                animation: "walking".to_string(),
                start: 0.0,
            })
            .with(Scale(Vec2::new(1.0, 1.0)))
            .with(Static)
            .build();
        Self {
            world,
            tick_dispatcher,
        }
    }

    fn load_level(_level: Level) {}

    fn tick(&mut self, dt: Duration, elapsed: Duration) -> Result<(), EngineError> {
        *self.world.write_resource::<DeltaTime>() = DeltaTime(dt);
        *self.world.write_resource::<ElapsedTime>() = ElapsedTime(elapsed);
        self.tick_dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        Ok(())
    }
}
