use super::components::*;

use crate::events::{Event, Key, Keyboard};
use crate::io;
use crate::math::{Mat3, Vec2};
use crate::resources::{self, registry, GetStore};
use crate::EngineError;
use core::iter::FromIterator;
use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use specs::prelude::*;
use specs_hierarchy::Hierarchy;

lazy_static::lazy_static! {
    pub static ref KEYBOARD: Keyboard = Keyboard::new();
}

pub struct EventSystem;
pub struct VelocitySystem;
pub struct GravitationSystem;
pub struct RenderSystem;
pub struct MovementSystem;
pub struct CheckPresentSystem;
pub struct UpdateAnimationSystem;

#[derive(Default)]
pub struct TextRenderSystem {
    pub modified: BitSet,
    pub processed: BitSet,
    pub reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for TextRenderSystem {
    type SystemData = (
        ReadStorage<'a, TextBox>,
        ReadStorage<'a, Present>,
        WriteStorage<'a, Parent>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Scale>,
        WriteStorage<'a, Glyph>,
        ReadExpect<'a, Hierarchy<Parent>>,
        Entities<'a>,
        Read<'a, RenderBufferDimensions>,
    );

    fn run(
        &mut self,
        (
            textboxes,
            present,
            mut parent,
            mut sprites,
            mut pos,
            mut scale,
            mut glyps,
            hierarchy,
            entities,
            buf_dim,
        ): Self::SystemData,
    ) {
        let events = textboxes.channel().read(self.reader_id.as_mut().unwrap());
        self.processed.clear();
        self.modified.clear();

        for event in events {
            match event {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    log::debug!("texbox modified: {}", id);
                    self.modified.add(*id);
                }
                ComponentEvent::Removed(_) => (),
            }
        }

        for (tex, entity, _) in (&textboxes, &self.modified, &present).join() {
            let layout_settings = LayoutSettings {
                x: 0.0,
                y: 0.0,
                max_width: tex.width,
                max_height: tex.height,
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Top,
                wrap_style: WrapStyle::Word,
                wrap_hard_breaks: true,
            };
            let entity = entities.entity(entity);
            let mut layout: Layout<()> = Layout::new(CoordinateSystem::PositiveYUp);
            layout.reset(&layout_settings);
            assert!(matches!(tex.font.variant, registry::ResourceVariant::Font));
            let res = &mut *resources::RESOURCE_TABLE.write();
            let font: Result<&mut resources::Font, EngineError> = res.get_mut(tex.font.id as usize);
            let style = TextStyle::new(&tex.content, tex.fontsize, 0);
            if let Ok(font) = font {
                layout.append(&[font.font()], &style);
                let mut ci = hierarchy.children(entity).iter();

                let curr_pos = pos
                    .get(entity)
                    .expect("Every textbox has to have a position")
                    .0;
                for glyph in layout.glyphs() {
                    if glyph.char_data.is_whitespace()
                        || glyph.char_data.is_control()
                        || glyph.char_data.is_missing()
                    {
                        continue;
                    }
                    let id = font.store_glyph(glyph);
                    let nscale = (
                        -(glyph.width as f32 / buf_dim.0 .0 as f32),
                        -(glyph.height as f32 / buf_dim.0 .1 as f32),
                    );
                    let (npo, nsp, nsc) = (
                        Pos(Vec2::new(
                            curr_pos.x() + glyph.x / buf_dim.0 .0 as f32 * 2.0 - nscale.0 / 2.0,
                            curr_pos.y() + glyph.y / buf_dim.0 .1 as f32 * 2.0 - nscale.1 / 2.0,
                        )),
                        Sprite {
                            id: tex.font.id,
                            sub_id: id,
                        },
                        Scale(Vec2::new(nscale.0, nscale.1)),
                    );
                    log::debug!("printing: {:?} pos: {:?}, scale: {:?}", glyph, npo, nsc);
                    match ci.next().cloned() {
                        None => {
                            entities
                                .build_entity()
                                .with(npo, &mut pos)
                                .with(nsp, &mut sprites)
                                .with(nsc, &mut scale)
                                .with(Glyph, &mut glyps)
                                .with(Parent { entity }, &mut parent)
                                .build();
                        }
                        Some(c) => {
                            if let Some((pos_, sprite, scale_)) =
                                (&mut pos, &mut sprites, &mut scale)
                                    .join()
                                    .get(c, &entities)
                            {
                                *pos_ = npo;
                                *sprite = nsp;
                                *scale_ = nsc;
                            } else {
                                unreachable!()
                            }
                        }
                    }
                }
                for id in ci {
                    entities.delete(*id).unwrap();
                }
            }
        }
        for (_, e) in (&glyps, &entities).join() {
            if parent.get(e).is_none() {
                entities.delete(e).unwrap();
            }
        }
        self.modified ^= &self.processed;
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(WriteStorage::<TextBox>::fetch(&world).register_reader());
    }
}

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut pos, vel, dt): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0 * dt.0.as_secs_f32();
        }
    }
}

impl<'a> System<'a> for GravitationSystem {
    type SystemData = (
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Static>,
        Read<'a, Gravitation>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut vel, is_static, g, dt): Self::SystemData) {
        for (vel, ()) in (&mut vel, !&is_static).join() {
            vel.0 += g.0 * dt.0.as_secs_f32();
        }
    }
}

impl<'a> System<'a> for UpdateAnimationSystem {
    type SystemData = (
        WriteStorage<'a, Animation>,
        ReadStorage<'a, Present>,
        Read<'a, ElapsedTime>,
    );

    fn run(&mut self, (mut animations, present, elapsed): Self::SystemData) {
        let res = &mut *resources::RESOURCE_TABLE.write();
        for (mut animation, _) in (&mut animations, &present).join() {
            let cha: Result<&mut Box<resources::Character>, EngineError> =
                res.get_mut(animation.id as usize);
            if let Ok(cha) = cha {
                if cha.animation_name() != animation.animation {
                    cha.set_animation(
                        animation.animation.as_str(),
                        elapsed.0.as_secs_f32() - animation.start,
                        0.0,
                        0.2, // fade time TODO make adjustable
                    )
                    .unwrap();
                    animation.start = elapsed.0.as_secs_f32();
                }
            }
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Animation>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, Present>,
        Read<'a, ElapsedTime>,
        Write<'a, SystemApi>,
        Write<'a, TextureIds>,
    );

    fn run(
        &mut self,
        (pos, sprite, anim, scale, present, elapsed, mut sys, mut tex_ids): Self::SystemData,
    ) {
        let mut sprites = Vec::new();
        for (pos, sprite, scale, _) in (&pos, &sprite, &scale, &present).join() {
            sprites.push(resources::Sprite::new(
                Mat3::translation(pos.0.x(), pos.0.y()) * Mat3::scaling(scale.0.x(), scale.0.y()),
                sprite.id,
                sprite.sub_id,
            ))
        }
        let res = &*resources::RESOURCE_TABLE.read();
        for (pos, anim, scale, _) in (&pos, &anim, &scale, &present).join() {
            let cha: Result<&Box<resources::Character>, EngineError> = res.get(anim.id as usize);
            if let Ok(cha) = cha {
                let trans = Mat3::translation(pos.0.x(), pos.0.y());
                let scale = Mat3::scaling(scale.0.x(), scale.0.y());

                match cha.interpolate(elapsed.0.as_secs_f32() - anim.start) {
                    Ok(sps) => {
                        for sp in sps.flatten() {
                            sprites.push(resources::Sprite::new(
                                trans * scale * sp.transform,
                                anim.id,
                                sp.att_id,
                            ))
                        }
                    }
                    Err(e) => log::error!("{}", e),
                }
            }
        }
        let mut dirty = false;
        for sp in &sprites {
            if !tex_ids.0.contains(&sp.tex_id) {
                tex_ids.0.push(sp.tex_id);
                dirty = true;
            }
        }
        if dirty {
            sys.0.push_textures(tex_ids.0.clone());
        }
        sys.0.push_sprites(sprites);
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Scale>,
        ReadStorage<'a, Speed>,
    );

    fn run(&mut self, (mut anim, mut vel, mut scale, speed): Self::SystemData) {
        for (anim, vel, scale, speed) in (&mut anim, &mut vel, &mut scale, &speed).join() {
            anim.animation = if KEYBOARD.get(Key::ARROW_RIGHT) {
                scale.0 = Vec2::new(1.0, scale.0.y());
                vel.0 = Vec2::new(speed.0, 0.0);
                "walking".to_owned()
            } else if KEYBOARD.get(Key::ARROW_LEFT) {
                scale.0 = Vec2::new(-1.0, scale.0.y());
                vel.0 = Vec2::new(-speed.0, 0.0);
                "walking".to_owned()
            } else {
                vel.0 = Vec2::new(0.0, 0.0);
                "standing".to_owned()
            };
        }
    }
}

impl<'a> System<'a> for CheckPresentSystem {
    type SystemData = (
        ReadStorage<'a, Animation>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, TextBox>,
        Entities<'a>,
        WriteStorage<'a, Present>,
    );

    fn run(&mut self, (anim, sprite, textbox, entities, mut present): Self::SystemData) {
        let res = &*resources::RESOURCE_TABLE.read();

        let mut modified = Vec::new();
        for (sprite, entity, _) in (&sprite, &entities, !&present).join() {
            if res.resource_present(sprite.id as usize) {
                modified.push(entity);
            }
        }
        for (anim, entity, _) in (&anim, &entities, !&present).join() {
            if res.resource_present(anim.id as usize) {
                modified.push(entity);
            }
        }
        for (textbox, entity, _) in (&textbox, &entities, !&present).join() {
            if res.resource_present(textbox.font.id as usize) {
                modified.push(entity);
            }
        }
        for item in modified {
            let _ = present
                .insert(item, Present)
                .map_err(|e| log::debug!("{}", e));
        }
    }
}

impl<'a> System<'a> for EventSystem {
    type SystemData = (Write<'a, SystemApi>, WriteStorage<'a, TextBox>);

    fn run(&mut self, (mut sys, mut textboxes): Self::SystemData) {
        let sys = &mut *sys.0;
        loop {
            let message = sys.poll_message().unwrap();
            match message {
                io::Message::None => break,
                io::Message::SystemInternal => continue,
                io::Message::Event(event) => {
                    log::trace!("event: {:?}", event);
                    match event {
                        Event::KeyDown(_, Key::KEY_P) => sys.play_sound(registry::SOUND.id),
                        Event::KeyDown(_, Key::KEY_S) => sys.stop_sound(registry::SOUND.id),
                        Event::KeyDown(_, Key::DIGIT1) => {
                            log::set_max_level(log::LevelFilter::Info)
                        }
                        Event::KeyDown(_, Key::DIGIT2) => {
                            log::set_max_level(log::LevelFilter::Debug)
                        }
                        Event::KeyDown(_, Key::DIGIT3) => {
                            log::set_max_level(log::LevelFilter::Trace)
                        }
                        Event::KeyDown(_, Key::KEY_A) => {
                            for t in (&mut textboxes).join() {
                                t.content += "a"
                            }
                        }
                        Event::KeyDown(_, key) => KEYBOARD.set(key, true),
                        Event::KeyUp(_, key) => KEYBOARD.set(key, false),
                        _ => (),
                    }
                }
            }
        }
    }
}
