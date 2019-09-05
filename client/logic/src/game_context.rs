use rask_engine::math;
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::sprite::*;
use rask_wasm_shared::state::State;

pub struct GameContext {
    state: State,
    tick_nr: u64,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            state: State::default(),
            tick_nr: 0
        })
    }

    fn push_state(&mut self) -> Result<(), ClientError> {
        let mut writer = get_double_buffer().borrow_writer();
        writer.set(self.state);
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        if self.state.sprites().is_empty() {
            self.state.append_sprite(
                &Sprite::new(math::Vec2::new(0.0, 0.0), 0, 0)
            );
            self.state.append_sprite(
                &Sprite::new(math::Vec2::new(0.0, 0.0), 1, 0)
            );
            *rask_wasm_shared::mem::shared_heap().animations_mut() = vec![
                Animation::new(vec![
                    Frame::new(vec![
                        rask_engine::math::Mat3::translation(0.5, 0.0) *
                        rask_engine::math::Mat3::scaling(0.2, 0.2)
                    ]),
                    Frame::new(vec![
                        rask_engine::math::Mat3::translation(0.5, 0.0) *
                        rask_engine::math::Mat3::scaling(0.1, 0.3)
                    ]),
                ]),
                Animation::new(vec![
                    Frame::new(vec![
                        rask_engine::math::Mat3::translation(-0.5, 0.0) *
                        rask_engine::math::Mat3::scaling(0.2, 0.2)
                    ]),
                    Frame::new(vec![
                        rask_engine::math::Mat3::translation(-0.5, 0.0) *
                        rask_engine::math::Mat3::scaling(0.3, 0.1)
                    ]),
                ]),
            ];
        }

        let animations = rask_wasm_shared::mem::shared_heap().animations();
        for sprite in self.state.sprites_mut().iter_mut() {
            if (self.tick_nr % 10) == 9 {
                sprite.next_frame(animations)
                    .ok_or(ClientError::ResourceError(format!("invalid animation id")))?;
            }
        }
        self.push_state()?;
        self.tick_nr += 1;
        Ok(())
    }
}

static mut GAME_CONTEXT: Option<GameContext> = None;

pub fn set_context(context: GameContext) {
    unsafe { GAME_CONTEXT = Some(context) }
}

pub fn context_mut() -> &'static mut GameContext {
    unsafe { GAME_CONTEXT.as_mut().unwrap() }
}

/* pub fn context() -> &'static Context {
    unsafe { CONTEXT.as_ref().unwrap() }
} */
