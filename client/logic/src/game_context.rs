use rask_engine::events::Event;
use rask_engine::resources::{registry, GetStore, ResourceTable, Texture, TextureIds};
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::mem::{RESOURCE_TABLE, RESOURCE_TABLE_ELEMENT_COUNT};
use rask_wasm_shared::sprite::*;
use rask_wasm_shared::{
    message_queue::{Message, MessageQueueReader},
    state::State,
};

const IMAGE1_DATA: &[u8] = include_bytes!("../../res/empty.png");
const IMAGE2_DATA: &[u8] = include_bytes!("../../res/thief.png");

pub struct GameContext {
    state: State,
    tick_nr: u64,
    #[allow(dead_code)]
    resource_table: ResourceTable,
    message_queue: MessageQueueReader,
}

impl GameContext {
    pub fn new() -> Result<Self, ClientError> {
        let resource_table = unsafe {
            let mut resource_table =
                ResourceTable::from_memory(RESOURCE_TABLE, RESOURCE_TABLE_ELEMENT_COUNT);
            resource_table.clear();
            resource_table.store(
                Texture::from_png_stream(IMAGE1_DATA)?,
                registry::IMAGE1.id as usize,
            )?;
            resource_table.store(
                Texture::from_png_stream(IMAGE2_DATA)?,
                registry::IMAGE2.id as usize,
            )?;
            resource_table.store(
                TextureIds {
                    reset_notify: 1,
                    ids: vec![registry::IMAGE1.id, registry::IMAGE2.id],
                },
                registry::USED_TEXTURE_IDS.id as usize,
            )?;
            resource_table
        };
        Ok(Self {
            state: State::default(),
            tick_nr: 0,
            resource_table,
            message_queue: MessageQueueReader::new(),
        })
    }

    fn push_state(&mut self) -> Result<(), ClientError> {
        let mut writer = get_double_buffer().borrow_writer();
        writer.set(self.state);
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        if self.state.sprites().is_empty() {
            let mut sprite = Sprite::default();
            self.state.append_sprite(&sprite);
            sprite.tex_id += 1;
            self.state.append_sprite(&sprite);
        }
        loop {
            let msg = self.message_queue.pop::<Message>();
            if let Message::None = msg {
                break;
            }
            log::info!("{:?}", msg);
            GameContext::handle_message(msg: Message)?;
        }

        self.push_state()?;
        self.tick_nr += 1;
        Ok(())
    }
    fn handle_message(message: Message) -> Result<Option<rask_engine::events::Event>, ClientError> {
        match message {
            Message::KeyDown(modifier, hash) => Ok(Some(Event::KeyDown)),
            Message::KeyUp(modifier, hash) => Ok(Some(Event::KeyDown)),
            _ => Err(ClientError::EngineError("Unknown Message Type".into())),
        }
    }
}
