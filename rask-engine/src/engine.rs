use crate::events::Event;
use crate::math::Vec2;
use crate::EngineError;
use specs::{prelude::*, Component};

const GRAVITY: Vec2 = Vec2::new(0.0, -9.807);

pub struct Level {}

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
    fn tick(&mut self, dt: core::time::Duration) -> Result<(), EngineError>;
}

/// The rask specific implementation of the `GameEngine`
#[allow(dead_code)]
pub struct RaskEngine {
    world: World,
    tick_dispatcher: Dispatcher<'static, 'static>,
}

#[derive(Debug, Default)]
struct Gravitation(Vec2);

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Vel(Vec2);

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Pos(Vec2);

#[derive(Debug, Default, Clone, Copy, Component)]
#[storage(NullStorage)]
struct Static;

struct VelocitySystem;
struct GravitationSystem;

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0;
        }
    }
}

impl<'a> System<'a> for GravitationSystem {
    type SystemData = (
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Static>,
        Read<'a, Gravitation>,
    );

    fn run(&mut self, (mut vel, is_static, g): Self::SystemData) {
        for (vel, ()) in (&mut vel, !&is_static).join() {
            vel.0 += g.0;
        }
    }
}

impl GameEngine for RaskEngine {
    fn new() -> Self {
        let mut world: specs::World = specs::WorldExt::new();
        world.insert(Gravitation(GRAVITY));

        let mut tick_dispatcher = DispatcherBuilder::new()
            .with(GravitationSystem, "gravitation", &[])
            .with(VelocitySystem, "velocity", &["gravitation"])
            .build();

        tick_dispatcher.setup(&mut world);
        Self {
            world,
            tick_dispatcher,
        }
    }

    fn load_level(_level: Level) {}

    fn handle_event(&mut self, _event: Event) -> Result<(), EngineError> {
        // TODO: Do dispatch_par
        self.tick_dispatcher.dispatch_seq(&mut self.world);
        self.world.maintain();
        Ok(())
    }

    fn tick(&mut self, _dt: core::time::Duration) -> Result<(), EngineError> {
        Ok(())
    }
}
