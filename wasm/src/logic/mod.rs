//! The GameContext contains the logic state and game engine.
//! Its main purpose is to handle events and execute the game engine.

mod resource_parser;
use crate::{
    communication::{Message, MessageQueue, Sprite, DOUBLE_BUFFER, SYNCHRONIZATION_MEMORY},
    error::ClientError,
};
use rask_engine::{
    engine::{GameEngine, RaskEngine},
    events::{Event, Key},
    resources::registry,
    resources::GetStore,
};
use resource_parser::ResourceParser;

pub struct LogicContext {
    engine: RaskEngine,
    last_timestamp: i32,
    state: Vec<Sprite>,
    tick_nr: u64,
    pub message_queue: MessageQueue,
    res_parser: ResourceParser,
    angle: i32,
    angle_mod: i32,
    anim_tick_nr: u32,
}

/// The logic context stores everything necessary for event handling and the game engine.
impl LogicContext {
    pub fn new() -> Result<Self, ClientError> {
        let mut res_parser = ResourceParser::new();
        res_parser.fetch_resource(registry::EMPTY)?;
        res_parser.fetch_resource(registry::THIEF)?;
        res_parser.fetch_resource(registry::SOUND)?;
        res_parser.fetch_character_resource(registry::CHAR)?;
        Ok(Self {
            engine: RaskEngine::new(),
            last_timestamp: unsafe { SYNCHRONIZATION_MEMORY.elapsed_ms },
            state: Vec::new(),
            tick_nr: 0,
            message_queue: MessageQueue::new(),
            res_parser,
            angle: 0,
            angle_mod: 0,
            anim_tick_nr: 0,
        })
    }

    fn push_state(&mut self) {
        let mut writer = DOUBLE_BUFFER.lock();
        *writer = self.state.clone();
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        let mut event = None;
        loop {
            let msg = self.message_queue.pop();
            if let Message::None = msg {
                break;
            }
            log::debug!("{:?}", msg);
            event = self.handle_message(msg)?;
        }
        if let Some(ref event) = event {
            self.engine.handle_event(event.clone())?;
        }
        match event {
            Some(Event::KeyDown(_, Key::ARROW_LEFT)) => self.angle_mod = -1,
            Some(Event::KeyDown(_, Key::ARROW_RIGHT)) => self.angle_mod = 1,
            Some(Event::KeyUp(_, Key::ARROW_RIGHT)) => self.angle_mod = 0,
            Some(Event::KeyUp(_, Key::ARROW_LEFT)) => self.angle_mod = 0,
            Some(Event::KeyDown(_, Key::KEY_P)) => Message::PlaySound(registry::SOUND.id).send(),
            Some(Event::KeyDown(_, Key::KEY_S)) => Message::StopSound(registry::SOUND.id).send(),
            Some(Event::KeyDown(_, Key::DIGIT1)) => log::set_max_level(log::LevelFilter::Info),
            Some(Event::KeyDown(_, Key::DIGIT2)) => log::set_max_level(log::LevelFilter::Debug),
            Some(Event::KeyDown(_, Key::DIGIT3)) => log::set_max_level(log::LevelFilter::Trace),
            Some(Event::KeyDown(_, Key::ENTER)) => {
                self.res_parser.fetch_resource(registry::EMPTY)?;
                self.res_parser.fetch_resource(registry::THIEF)?;
                self.res_parser.fetch_character_resource(registry::CHAR)?;
                self.res_parser.fetch_resource(registry::SOUND)?;
            }
            Some(Event::KeyDown(_, Key::KEY_W)) => {
                let mut src = crate::communication::SCREEN_RECT_SCALE.write();
                if *src > 1.025 {
                    *src -= 0.05;
                }
            }
            Some(Event::KeyDown(_, Key::KEY_E)) => {
                let mut src = crate::communication::SCREEN_RECT_SCALE.write();
                if *src < 1.975 {
                    *src += 0.05;
                }
            }
            _ => (),
        }
        self.angle += self.angle_mod;

        // TODO: Remove this temporary sprite loading. Replace it with some kind of
        // "resource complete" event
        if self.state.len() < 2 {
            let res = crate::communication::RESOURCE_TABLE.read();
            let texid1 = registry::EMPTY.id;
            let texid2 = registry::THIEF.id;
            let charid = registry::CHAR.id;
            let tex1: Result<&rask_engine::resources::Texture, _> = res.get(texid1 as usize);
            let tex2: Result<&rask_engine::resources::Texture, _> = res.get(texid2 as usize);
            let charc: Result<&Box<rask_engine::resources::Character>, _> =
                res.get(charid as usize);
            if let (Ok(_), Ok(_), Ok(charc)) = (tex1, tex2, charc) {
                log::info!("loaded all resoucres");
                let mut guard = crate::communication::TEXTURE_IDS.lock();
                for &(id, mat) in &[
                    (texid1, rask_engine::math::Mat3::identity()),
                    (texid2, rask_engine::math::Mat3::identity()),
                ] {
                    guard.ids.push(id);
                    self.state
                        .push(crate::communication::Sprite::new(mat, id, 0));
                }
                let sprites = charc.interpolate(0.0, "walking")?;
                guard.ids.push(charid);
                for sprite in sprites {
                    self.state
                        .push(crate::communication::Sprite::from_animation_state(
                            sprite?, charid,
                        ));
                }
                guard.reset_notify = 1;
            }
        }
        log::trace!("angle: {}", self.angle);
        if self.state.len() >= 3 {
            self.state[1].transform = rask_engine::math::Mat3::rotation(0.02 * self.angle as f32)
                * rask_engine::math::Mat3::scaling(0.0, 0.0);
            let res = crate::communication::RESOURCE_TABLE.read();
            let charid = rask_engine::resources::registry::CHAR.id;
            let charc: &Box<rask_engine::resources::Character> = res.get(charid as usize).unwrap();
            let sprites = charc.interpolate(self.tick_nr as f32 * 0.018, "walking")?;
            for (i, sprite) in sprites.enumerate() {
                self.state[2 + i] =
                    crate::communication::Sprite::from_animation_state(sprite?, charid);
                self.state[2 + i].transform = rask_engine::math::Mat3::translation(
                    self.tick_nr as f32 / (90.0 * 2.0) % 2.2 - 1.1,
                    -0.72,
                ) * self.state[2 + i].transform;
            }
        }

        let now = unsafe { SYNCHRONIZATION_MEMORY.elapsed_ms };
        self.engine.tick(core::time::Duration::from_millis(
            (now - self.last_timestamp) as u64,
        ));
        self.last_timestamp = now;

        self.push_state();
        self.tick_nr += 1;
        Ok(())
    }

    fn handle_message(&mut self, message: Message) -> Result<Option<Event>, ClientError> {
        match message {
            Message::KeyDown(modifier, hash) => Ok(Some(Event::KeyDown(modifier, Key::from(hash)))),
            Message::KeyUp(modifier, hash) => Ok(Some(Event::KeyUp(modifier, Key::from(hash)))),
            Message::MouseDown(event) => Ok(Some(Event::MouseDown(event))),
            Message::MouseUp(event) => Ok(Some(Event::MouseUp(event))),
            Message::KeyPress(t, code) => Ok(Some(Event::KeyPress(t as u16, code))),
            Message::AudioLoaded(id) => {
                let mut res = crate::communication::RESOURCE_TABLE.write();
                res.store(rask_engine::resources::Sound, id as usize)?;
                Ok(None)
            }
            Message::RequestAlloc { id, size } => self.res_parser.alloc(id, size).map(|_| None),
            Message::DoneWritingResource(id) => self.res_parser.parse(id).map(|_| None),
            _ => Err(ClientError::EngineError("Unknown Message Type".into())),
        }
    }
}
