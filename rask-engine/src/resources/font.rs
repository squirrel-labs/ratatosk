use super::Texture;
use crate::error::EngineError;
use fontdue::{layout::GlyphPosition, Font as FFont, FontSettings, Metrics};
use lru::LruCache;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const FONTCACHESIZE: usize = 256;

pub struct Font {
    font: FFont,
    cache: LruCache<u64, (Metrics, Texture)>,
}

impl Font {
    pub fn new(data: &[u8]) -> Result<Self, EngineError> {
        let font = FFont::from_bytes(data, FontSettings::default())?;

        Ok(Self {
            font,
            cache: LruCache::new(FONTCACHESIZE),
        })
    }

    /// Stores the glyph in the cache and return the texture subid and whether the it was already
    /// present in the cache
    pub fn store_glyph(&mut self, glyph: &GlyphPosition) -> (u64, bool) {
        let mut s = DefaultHasher::new();
        glyph.key.hash(&mut s);
        let key = s.finish();
        //log::debug!("adding key: {}, glyph: {:?}", key, glyph);
        if !self.cache.contains(&key) {
            let (metrics, data) = self.font.rasterize_config(glyph.key);
            let mut new_data = Vec::new(); // TODO: Replace with color management
            for d in data {
                new_data.append(&mut vec![d, d, d, d]);
            }
            let tex = Texture::form_raw_parts(
                new_data,
                metrics.width as u32,
                metrics.height as u32,
                image::ColorType::Rgba8,
            );

            self.cache.put(key, (metrics, tex));
            (key, true)
        } else {
            (key, false)
        }
    }

    pub fn get_glyph_metrics(&self, key: u64) -> Option<&Metrics> {
        self.cache.peek(&key).map(|x| &x.0)
    }
    pub fn get_glyph_data(&self, key: u64) -> Option<&Texture> {
        self.cache.peek(&key).map(|x| &x.1)
    }

    pub fn cache(&self) -> &LruCache<u64, (Metrics, Texture)> {
        &self.cache
    }
    pub fn font(&self) -> &FFont {
        &self.font
    }
}
