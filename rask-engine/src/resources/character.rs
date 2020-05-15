use super::Texture;
use crate::network::packet::ResourceData;
use crate::{math::Mat3, EngineError};
use spine::atlas::Atlas;
use spine::atlas::Texture as TextureSegment;
use spine::skeleton::{Skeleton, SRT};
use std::convert::TryFrom;

use std::collections::HashMap;
use std::io::Read;

struct OwnedSpriteState {
    attachment: String,
    srt: SRT,
}

#[allow(dead_code)]
pub struct AnimationState {
    /// if true, the image has to be rotated clockwise
    rotated: bool,
    /// position of the upper left pixel in the texture segment
    pos: (u16, u16),
    /// size of the texture segment
    size: (u16, u16),
    /// transformation matrix for the subsprite
    transform: Mat3,
}

pub struct AnimationStates<'a> {
    sprites: std::vec::IntoIter<OwnedSpriteState>,
    atlas: &'a HashMap<String, TextureSegment>,
}

impl<'a> AnimationStates<'a> {
    fn new(
        sprites: std::vec::IntoIter<OwnedSpriteState>,
        atlas: &'a HashMap<String, TextureSegment>,
    ) -> Self {
        Self { sprites, atlas }
    }
}

impl<'a> Iterator for AnimationStates<'a> {
    type Item = Result<AnimationState, EngineError>;
    fn next(&mut self) -> Option<Self::Item> {
        let sprite = self.sprites.next()?;
        if let Some(region) = self.atlas.get(&sprite.attachment) {
            Some(Ok(AnimationState {
                rotated: region.rotate,
                pos: region.xy,
                size: region.size,
                transform: Mat3::from(sprite.srt),
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
    texture: Texture,
    skeleton: Skeleton,
    atlas: HashMap<String, TextureSegment>,
}

impl Character {
    pub fn new<R: Read>(
        texture: Texture,
        skeleton: Skeleton,
        atlas: Atlas<R>,
    ) -> Result<Self, EngineError> {
        let mut segments = HashMap::new();
        for segment in atlas {
            let segment = segment.map_err(|e| {
                EngineError::ResourceFormat(format!("Could not parse atlas \"{}\"", e))
            })?;
            segments.insert(segment.name.clone(), segment);
        }
        Ok(Self::from_parts(texture, skeleton, segments))
    }

    pub fn from_parts(
        texture: Texture,
        skeleton: Skeleton,
        atlas: HashMap<String, TextureSegment>,
    ) -> Self {
        Self {
            texture,
            skeleton,
            atlas,
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn skeleton(&self) -> &Skeleton {
        &self.skeleton
    }

    pub fn atlas(&self) -> &HashMap<String, TextureSegment> {
        &self.atlas
    }

    pub fn texture_mut(&mut self) -> &mut Texture {
        &mut self.texture
    }

    pub fn skeleton_mut(&mut self) -> &mut Skeleton {
        &mut self.skeleton
    }

    pub fn atlas_mut(&mut self) -> &mut HashMap<String, TextureSegment> {
        &mut self.atlas
    }

    pub fn interpolate<'a>(
        &'a self,
        time: f32,
        anim_name: &str,
    ) -> Result<AnimationStates<'a>, EngineError> {
        let animated_skin = self
            .skeleton
            .get_animated_skin("default", Some(anim_name))?;
        Ok(AnimationStates::new(
            animated_skin
                .interpolate(time)
                .ok_or_else(|| {
                    EngineError::Animation(format!(
                        "Could not interpolate animation at time {}",
                        time
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
            let texture = Texture::from_png_stream(&data[0..texture_len as usize]);
            let atlas =
                spine::atlas::Atlas::from_reader(&data[texture_len as usize..atlas_len as usize]);
            let skeleton = spine::skeleton::Skeleton::from_reader(
                &data[(texture_len + atlas_len) as usize..animation_len as usize],
            );
            Character::new(texture?, skeleton?, atlas?)
        } else {
            Err(EngineError::ResourceFormat(
                "The given data is not a character variant".into(),
            ))
        }
    }
}
