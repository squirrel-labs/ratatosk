use rask_engine::math;
use rask_engine::resources::{GetStore, ResourceTable, Texture};
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::mem::{RESOURCE_TABLE, RESOURCE_TABLE_ELEMENT_COUNT};
use rask_wasm_shared::sprite::*;
use rask_wasm_shared::state::State;

const IMAGE1_DATA: &[u8] = include_bytes!("../../res/empty.png");
const IMAGE2_DATA: &[u8] = include_bytes!("../../res/thief.png");

pub struct GameContext {
    state: State,
    tick_nr: u64,
    resource_table: ResourceTable,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            state: State::default(),
            tick_nr: 0,
            resource_table: unsafe {
                ResourceTable::new(RESOURCE_TABLE, RESOURCE_TABLE_ELEMENT_COUNT)
            },
        })
    }

    fn push_state(&mut self) -> Result<(), ClientError> {
        let mut writer = get_double_buffer().borrow_writer();
        writer.set(self.state);
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        if self.state.sprites().is_empty() {
            self.state.append_sprite(&Sprite::default());

            unsafe {
                self.resource_table
                    .store(Texture::from_png_stream(IMAGE1_DATA)?, 0)?;
                self.resource_table
                    .store(Texture::from_png_stream(IMAGE2_DATA)?, 1)?;
            }
        }

        self.push_state()?;
        self.tick_nr += 1;
        Ok(())
    }
}
