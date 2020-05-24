use crate::communication::Sprite;
use crate::error::ClientError;
use rask_engine::resources::texture::Texture;

pub trait GraphicsApi: Sized {
    type GraphicsError: std::fmt::Display;

    /// Create a new graphics context.
    /// `width` and `height` is the resolution of the pixelized image.
    fn new(width: u32, height: u32) -> Result<Self, ClientError>;

    /// Update the sprite vector to the given slice of sprites
    fn update_sprite_vector(&mut self, sprites: &[Sprite]) -> Result<(), ClientError>;

    /// Upload the given id-to-texture mapping to the graphics context
    fn upload_textures(&mut self, textures: &[(u32, &Texture)]) -> Result<(), ClientError>;

    /// Reove all textures from the graphics context
    fn remove_textures(&mut self) -> Result<(), ClientError>;

    /// Draw all sprites from the current sprite vector
    fn draw(&mut self) -> Result<(), ClientError>;

    /// Force a canvas resizing to a given resolution
    fn set_size(&mut self, w: u32, h: u32);

    /// Set the canvas size to given resolution.
    /// Do not update size if it has not changed.
    fn update_size(&mut self, w: u32, h: u32);

    /// Get the last graphics context error
    fn ok(&self) -> Result<(), Self::GraphicsError>;
}
