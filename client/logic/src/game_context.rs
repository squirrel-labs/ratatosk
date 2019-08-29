use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::sprite::Sprite;
use rask_wasm_shared::state::State;

pub struct GameContext {
    state: State,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            state: State::default(),
        })
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        if self.state.sprites().is_empty() {
            self.state.append_sprite(&Sprite::default());
            let mut writer = get_double_buffer().borrow_writer();
            writer.set(self.state);
        }
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
