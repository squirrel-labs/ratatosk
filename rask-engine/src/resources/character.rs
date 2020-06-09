use std::collections::HashMap;
use std::io::Read;

use super::Texture;
use crate::network::packet::ResourceData;
use crate::{math::Mat3, EngineError};
use image::DynamicImage;
use spine::atlas::Atlas;
use spine::skeleton::{Skeleton, SRT};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

struct OwnedSpriteState {
    attachment: String,
    srt: SRT,
}

#[allow(dead_code)]
pub struct AnimationState {
    /// transformation matrix for the subsprite
    pub transform: Mat3,
    /// attachment id
    pub att_id: u64,
}

pub struct AnimationStates<'a> {
    sprites: std::vec::IntoIter<OwnedSpriteState>,
    atlas: &'a HashMap<u64, Texture>,
}

impl<'a> AnimationStates<'a> {
    fn new(
        sprites: std::vec::IntoIter<OwnedSpriteState>,
        atlas: &'a HashMap<u64, Texture>,
    ) -> Self {
        Self { sprites, atlas }
    }
}

impl<'a> Iterator for AnimationStates<'a> {
    type Item = Result<AnimationState, EngineError>;
    fn next(&mut self) -> Option<Self::Item> {
        let sprite = self.sprites.next()?;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        sprite.attachment.hash(&mut hasher);
        let att_id = hasher.finish();
        if self.atlas.contains_key(&att_id) {
            Some(Ok(AnimationState {
                transform: Mat3::from(sprite.srt),
                att_id,
            }))
        } else {
            Some(Err(EngineError::ResourceMissing(format!(
                "Could not get sprite attachment \"{}\"",
                sprite.attachment
            ))))
        }
    }
}

pub struct Character {
    skeleton: Skeleton,
    atlas: HashMap<u64, Texture>,
}

impl Character {
    pub fn new<R: Read>(
        texture: DynamicImage,
        skeleton: Skeleton,
        atlas: Atlas<R>,
    ) -> Result<Self, EngineError> {
        let mut segments = HashMap::new();
        for segment in atlas {
            let segment = segment.map_err(|e| {
                EngineError::ResourceFormat(format!("Could not parse atlas \"{}\"", e))
            })?;
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            segment.name.hash(&mut hasher);
            let (x, y) = segment.xy;
            let (width, height) = segment.size;
            let tex = texture.crop_imm(x as u32, y as u32, width as u32, height as u32);
            if segment.rotate {
                tex.rotate90();
            }
            segments.insert(hasher.finish(), Texture::from_dynamic_image(tex));
        }
        Ok(Self::from_parts(skeleton, segments))
    }

    pub fn from_parts(skeleton: Skeleton, atlas: HashMap<u64, Texture>) -> Self {
        Self { skeleton, atlas }
    }

    pub fn skeleton(&self) -> &Skeleton {
        &self.skeleton
    }

    pub fn atlas(&self) -> &HashMap<u64, Texture> {
        &self.atlas
    }

    pub fn skeleton_mut(&mut self) -> &mut Skeleton {
        &mut self.skeleton
    }

    pub fn atlas_mut(&mut self) -> &mut HashMap<u64, Texture> {
        &mut self.atlas
    }

    pub fn interpolate(&self, time: f32, anim_name: &str) -> Result<AnimationStates, EngineError> {
        let animated_skin = self
            .skeleton
            .get_animated_skin("default", Some(anim_name))?;
        Ok(AnimationStates::new(
            animated_skin
                .interpolate(time)
                .ok_or_else(|| {
                    EngineError::Animation(format!(
                        "Could not interpolate animation at time {}",
                        time,
                    ))
                })?
                .map(|s| OwnedSpriteState {
                    attachment: s.attachment.to_owned(),
                    srt: s.srt,
                })
                .collect::<Vec<_>>()
                .into_iter(),
            &self.atlas,
        ))
    }
}

impl<'a> TryFrom<ResourceData<'a>> for Character {
    type Error = EngineError;
    fn try_from(chr_data: ResourceData<'a>) -> Result<Self, Self::Error> {
        if let ResourceData::CharacterVec {
            texture_len,
            atlas_len,
            animation_len,
            data,
        } = chr_data
        {
            let texture = image::load_from_memory_with_format(
                &data[0..texture_len as usize],
                image::ImageFormat::Png,
            );
            let atlas = spine::atlas::Atlas::from_reader(
                &data[texture_len as usize..(texture_len + atlas_len) as usize],
            );
            let skeleton = spine::skeleton::Skeleton::from_reader(
                &data[(texture_len + atlas_len) as usize
                    ..(texture_len + atlas_len + animation_len) as usize],
            );
            Character::new(texture?, skeleton?, atlas?)
        } else {
            Err(EngineError::ResourceFormat(
                "The given data is not a character variant".into(),
            ))
        }
    }
}
