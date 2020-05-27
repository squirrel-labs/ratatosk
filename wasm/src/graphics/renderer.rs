use super::webgl::WebGl2;
use super::GraphicsApi;
use crate::communication::SynchronizationMemory;
use crate::communication::RESOURCE_TABLE;
use crate::error::ClientError;
use rask_engine::resources::{GetTextures, Texture};

type RenderBackend = WebGl2;
static mut RENDERER: Option<Renderer<RenderBackend>> = None;

/// # Safety
///
/// This function is not thread safe.
pub unsafe fn set_renderer(
    renderer: Renderer<RenderBackend>,
) -> &'static mut Renderer<RenderBackend> {
    RENDERER = Some(renderer);
    RENDERER.as_mut().unwrap()
}

/// # Safety
///
/// This function is not thread safe.
pub unsafe fn renderer_mut() -> Option<&'static mut Renderer<RenderBackend>> {
    RENDERER.as_mut()
}

pub struct Renderer<T> {
    graphics: T,
}

impl<T: GraphicsApi> Renderer<T> {
    pub fn new() -> Result<Self, ClientError> {
        // TODO: Do not hardcode pixelated framebuffer size
        T::new(160, 90).map(|api| Self { graphics: api })
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        self.graphics
            .ok()
            .map_err(|e| ClientError::WebGlError(format!("WebGl2 error: {}", e)))?;
        let size = (unsafe { SynchronizationMemory::get() }).canvas_size;
        self.graphics.update_size(size.0, size.1);
        self.draw_sprites()
    }

    pub fn draw_sprites(&mut self) -> Result<(), ClientError> {
        let used_textures = crate::communication::TEXTURE_IDS.lock();
        if used_textures.reset_notify > 0 {
            self.graphics.remove_textures()?;
            let guard = RESOURCE_TABLE.read();
            let textures = used_textures
                .ids
                .iter()
                .map(|&id| (id, guard.get_textures(id as usize)))
                .map(|(id, tex): (_, Result<Vec<(u64, &Texture)>, _>)| {
                    tex.map(|tex: Vec<(u64, &Texture)>| {
                        Ok(tex
                            .iter()
                            .map(|(sid, t): &(u64, &Texture)| (id, *sid, *t))
                            .collect())
                    })
                })
                .flatten()
                .collect::<Result<Vec<(u32, u64, &Texture)>, _>>()?;
            self.graphics.upload_textures(&textures)?;
        }
        let state = *crate::communication::DOUBLE_BUFFER.lock();
        self.graphics.update_sprite_vector(state.sprites())?;
        self.graphics.draw()
    }
}
