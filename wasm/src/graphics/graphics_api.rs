use crate::error::ClientError;
use rask_engine::math::Mat3;
use rask_engine::resources::texture::Texture;

pub trait GraphicsApi: Sized {
    type GraphicsError: std::fmt::Display;

    /// Create a new graphics context.
    /// `width` and `height` is the resolution of the pixelized image.
    fn new(width: u32, height: u32) -> Result<Self, ClientError>;

    /// Start drawing on the context
    fn start_frame(&mut self) -> Result<(), ClientError>;

    /// Apply drawings on the context
    fn end_frame(&self) -> Result<(), ClientError>;

    /// Try to draw a rectangle with given transformation and texture.
    /// If the given texture isn't found return `Ok(None)`.
    fn draw_rect(&self, mat: &Mat3, tex: u32) -> Result<Option<()>, ClientError>;

    /// Upload a texture to the graphics context
    fn upload_texture(&mut self, texture: &Texture, id: u32) -> Result<(), ClientError>;

    /// Remove a texture from the graphics context
    fn unload_texture(&mut self, id: u32) -> Result<(), ClientError>;

    /// Force a canvas resizing to a given resolution
    fn set_size(&mut self, w: u32, h: u32);

    /// Set the canvas size to given resolution.
    /// Do not update size if it has not changed.
    fn update_size(&mut self, w: u32, h: u32);

    /// Get the last graphics context error
    fn ok(&self) -> Result<(), Self::GraphicsError>;
}
