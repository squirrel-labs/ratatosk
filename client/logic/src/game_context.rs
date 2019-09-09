use rask_engine::math;
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::sprite::*;
use rask_wasm_shared::state::State;

const IMAGE1_DATA: &[u8] = include_bytes!("../../res/kuh.png");
const IMAGE2_DATA: &[u8] = include_bytes!("../../res/mensch.png");
//const IMAGE1_DATA: &[u8] = include_bytes!("../../res/empty.png");

pub struct GameContext {
    state: State,
    tick_nr: u64,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            state: State::default(),
            tick_nr: 0,
        })
    }

    fn push_state(&mut self) -> Result<(), ClientError> {
        let mut writer = get_double_buffer().borrow_writer();
        writer.set(self.state);
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        if self.state.sprites().is_empty() {
            self.state
                .append_sprite(&Sprite::new(math::Vec2::new(0.0, 0.0), 3, 0, 0));
            self.state
                .append_sprite(&Sprite::new(math::Vec2::new(0.0, 0.0), 2, 0, 0));
            self.state
                .append_sprite(&Sprite::new(math::Vec2::new(0.3, 0.3), 0, 0, 1));
            self.state
                .append_sprite(&Sprite::new(math::Vec2::new(0.0, 0.0), 1, 0, 1));
            self.state
                .append_sprite(&Sprite::new(math::Vec2::new(0.0, -0.6), 0, 0, 1));
            self.state
                .append_sprite(&Sprite::new(math::Vec2::new(-0.6, 0.6), 1, 0, 1));

            let shared_heap = rask_wasm_shared::mem::shared_heap();
            *shared_heap.animations_mut() = vec![
                Animation::new(vec![
                    Frame::new(vec![rask_engine::math::Mat3::scaling(0.4, 0.4)]),
                    Frame::new(vec![
                        rask_engine::math::Mat3::scaling(0.4, 0.4)
                            * rask_engine::math::Mat3::translation(0.5, 0.0)
                            * rask_engine::math::Mat3::rotation(6.0),
                    ]),
                ]),
                Animation::new(vec![
                    Frame::new(vec![rask_engine::math::Mat3::scaling(0.4, 0.4)]),
                    Frame::new(vec![rask_engine::math::Mat3::scaling(0.6, 0.2)]),
                ]),
                Animation::new(vec![Frame::new(vec![rask_engine::math::Mat3::scaling(
                    9.0 / 16.0,
                    1.0,
                )])]),
                Animation::new(vec![Frame::new(vec![rask_engine::math::Mat3::identity()])]),
            ];

            *shared_heap.textures_mut() = Some(vec![
                rask_wasm_shared::texture::Texture::from_png_stream(IMAGE1_DATA)?,
                rask_wasm_shared::texture::Texture::from_png_stream(IMAGE2_DATA)?,
            ]);
            shared_heap.set_texture_notify();
        }

        let animations = rask_wasm_shared::mem::shared_heap().animations();
        for sprite in self.state.sprites_mut().iter_mut() {
            if (self.tick_nr % 10) == 9 {
                sprite
                    .next_frame(animations)
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
}*/
