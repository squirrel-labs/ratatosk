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
    state: Vec<Sprite>,
    tick_nr: u64,
    message_queue: usize,
}

/// The logic context stores everything necessary for event handling and the game engine.
impl LogicContext {
    pub fn new(pool: rayon::ThreadPool) -> Result<Self, ClientError> {
        let sys = SystemIO::new()?;
        let message_queue = &sys.message_queue as *const MessageQueue as *const u8 as usize;
        Ok(Self {
            engine: RaskEngine::new(std::sync::Arc::new(pool), Box::new(sys)),
            last_timestamp: unsafe { SYNCHRONIZATION_MEMORY.elapsed_ms },
            state: Vec::new(),
            tick_nr: 0,
            message_queue,
        })
    }

    fn push_state(&mut self) {
        let mut writer = DOUBLE_BUFFER.lock();
        *writer = self.state.clone();
    }

    pub fn get_message_queue_pos(&self) -> usize {
        self.message_queue
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        let now = unsafe { SYNCHRONIZATION_MEMORY.elapsed_ms };
        self.engine.tick(core::time::Duration::from_millis(
            (now - self.last_timestamp) as u64,
        ))?;
        self.last_timestamp = now;

        self.push_state();
        self.tick_nr += 1;
        Ok(())
    }
}
