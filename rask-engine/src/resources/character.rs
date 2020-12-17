use std::collections::HashMap;
use std::io::Read;

use super::Texture;
use crate::network::packet::ResourceData;
use crate::{math::Mat3, math::Vec3, EngineError};
use image::DynamicImage;
use spine::skeleton::Skeleton;
use spine::{atlas::Atlas, skeleton::animation::skin::SkinAnimation};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use ouroboros::self_referencing;

#[derive(Debug)]
struct OwnedSpriteState {
    attachment: String,
    transform: [[f32; 3]; 3],
}

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

impl AnimationState {
    fn new(transform: [[f32; 3]; 3], attachment_id: u64) -> Self {
        let tscale = Mat3::scaling(0.5 / 512.0, 0.5 / 512.0);
        let ascale = Mat3::scaling(1.2, 1.2);
        let mat = Mat3::from_vec3(
            Vec3::from(transform[0]),
            Vec3::from(transform[1]),
            Vec3::from(transform[2]),
        );
        Self {
            transform: tscale * mat * ascale,
            att_id: attachment_id,
        }
    }
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
            Some(Ok(AnimationState::new(sprite.transform, att_id)))
        } else {
            Some(Err(EngineError::ResourceMissing(format!(
                "Could not get sprite attachment \"{}\"",
                sprite.attachment
            ))))
        }
    }
}

#[self_referencing]
struct Skel {
    pub skeleton: Box<Skeleton>,
    animation_name: String,
    #[borrows(skeleton)]
    animation: Option<SkinAnimation<'this>>,
}

pub struct Character {
    atlas: HashMap<u64, Texture>,
    animation: Skel,
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
        let skel = SkelBuilder {
            skeleton: Box::new(skeleton),
            animation_name: "".to_owned(),
            animation_builder: |_| None,
        }
        .build();

        Self {
            atlas,
            animation: skel,
        }
    }

    pub fn from_memory(
        texture: &[u8],
        animation: &[u8],
        atlas: &[u8],
    ) -> Result<Self, EngineError> {
        let texture = image::load_from_memory_with_format(texture, image::ImageFormat::Png);
        let atlas = spine::atlas::Atlas::from_reader(atlas);
        let skeleton = spine::skeleton::Skeleton::from_reader(animation);
        Character::new(texture?, skeleton?, atlas?)
    }

    pub fn skeleton(&self) -> &Skeleton {
        &self.animation.with_skeleton_contents(|skeleton| skeleton)
    }

    pub fn atlas(&self) -> &HashMap<u64, Texture> {
        &self.atlas
    }

    pub fn atlas_mut(&mut self) -> &mut HashMap<u64, Texture> {
        &mut self.atlas
    }

    pub fn set_animation(
        &mut self,
        anim_name: &str,
        start_offset: f32,
        current_time: f32,
        fade_time: f32,
    ) -> Result<(), EngineError> {
        self.animation.with_mut(|fields| {
            *fields.animation = Some(
                if fields.animation.is_some() {
                    Skeleton::get_animated_skin_with_transiton(
                        &fields.skeleton_contents,
                        "default",
                        anim_name,
                        fields.animation_name.as_str(),
                        current_time,
                        start_offset,
                        fade_time,
                    )
                } else {
                    fields
                        .skeleton_contents
                        .get_animated_skin("default", Some(anim_name))
                }
                .unwrap(),
            )
        });
        Ok(())
    }
    pub fn interpolate(&self, time: f32, anim_name: &str) -> Result<AnimationStates, EngineError> {
        let animated_skin = self
            .skeleton()
            .get_animated_skin("default", Some(anim_name))?;
        let time = time.rem_euclid(animated_skin.get_duration());
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
                    transform: s.to_matrix3(),
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
            Character::from_memory(
                &data[0..texture_len as usize],
                &data[(texture_len + atlas_len) as usize
                    ..(texture_len + atlas_len + animation_len) as usize],
                &data[texture_len as usize..(atlas_len + texture_len) as usize],
            )
        } else {
            Err(EngineError::ResourceFormat(
                "The given data is not a character variant".into(),
            ))
        }
    }
}
