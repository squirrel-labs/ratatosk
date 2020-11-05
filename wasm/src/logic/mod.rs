//! The GameContext contains the logic state and game engine.
//! Its main purpose is to handle events and execute the game engine.

mod resource_parser;
mod system;
use crate::{
    communication::{MessageQueue, Sprite, DOUBLE_BUFFER, SYNCHRONIZATION_MEMORY},
    error::ClientError,
};
use rask_engine::engine::{GameEngine, RaskEngine};
use system::SystemIO;

pub struct LogicContext {
    engine: RaskEngine,
    last_timestamp: i32,
    tick_nr: u64,
    message_queue: usize,
}

/// The logic context stores everything necessary for event handling and the game engine.
impl LogicContext {
    pub fn new(pool: rayon::ThreadPool) -> Result<Self, ClientError> {
        let sys = Box::new(SystemIO::new()?);
        let message_queue = sys.message_queue.pos();
        Ok(Self {
            engine: RaskEngine::new(std::sync::Arc::new(pool), sys),
            last_timestamp: unsafe { SYNCHRONIZATION_MEMORY.elapsed_ms },
            tick_nr: 0,
            message_queue,
        })
    }

    pub fn get_message_queue_pos(&self) -> usize {
        self.message_queue
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        let now = unsafe { SYNCHRONIZATION_MEMORY.elapsed_ms };
        self.engine.tick(
            core::time::Duration::from_millis((now - self.last_timestamp) as u64),
            core::time::Duration::from_millis(now as u64),
        )?;
        self.last_timestamp = now;

        self.tick_nr += 1;
        Ok(())
    }
}
