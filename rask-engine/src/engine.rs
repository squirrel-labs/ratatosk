use crate::events::Event;
use crate::resources::ResourceTable;
use crate::world::World;
use crate::EngineError;

/// An interface for the game server to interact with the game.
pub trait GameEngine {
    /// Create a new game.
    fn new() -> Self;

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

// TODO: impl GameEngine for RaskEngine {}
