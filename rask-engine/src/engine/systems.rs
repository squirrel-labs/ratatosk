use super::components::*;

use crate::events::Event;
use crate::events::Key;
use crate::io;
use crate::math::Mat3;
use crate::resources::{self, registry, GetStore};
use crate::EngineError;
use specs::prelude::*;

pub struct EventSystem;
pub struct VelocitySystem;
pub struct GravitationSystem;
pub struct RenderSystem;

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

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Sprite>,
        WriteStorage<'a, Animation>,
        Read<'a, ElapsedTime>,
        Write<'a, SystemApi>,
        Write<'a, TextureIds>,
    );

    fn run(&mut self, (pos, sprite, mut anim, elapsed, mut sys, mut tex_ids): Self::SystemData) {
        let mut sprites = Vec::new();
        for (pos, sprite) in (&pos, &sprite).join() {
            if res.resource_present(sprite.id as usize) {
                sprites.push(resources::Sprite::new(
                    Mat3::translation(pos.0.x(), pos.0.y())
                        * Mat3::scaling(sprite.scale_x, sprite.scale_y),
                    sprite.id,
                    0,
                ))
            }
        }
        let res = &mut *resources::RESOURCE_TABLE.write();
        for (pos, anim) in (&pos, &mut anim).join() {
            if res.resource_present(anim.id as usize) {
                let trans = Mat3::translation(pos.0.x(), pos.0.y());
                let cha: Result<&mut Box<resources::Character>, EngineError> =
                    res.get_mut(anim.id as usize);
                let cha = cha.unwrap().as_mut();

                //todo factor out into own subsystem
                if cha.animation_name() != anim.animation {
                    cha.set_animation(
                        anim.animation.as_str(),
                        0.0,
                        elapsed.0.as_secs_f32() - anim.start,
                        0.1,
                    )
                    .unwrap();
                }

                if let Ok(sps) = cha.interpolate(elapsed.0.as_secs_f32()) {
                    for sp in sps {
                        let sp = sp.unwrap();
                        sprites.push(resources::Sprite::new(
                            trans * sp.transform,
                            anim.id,
                            sp.att_id,
                        ))
                    }
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

impl<'a> System<'a> for EventSystem {
    type SystemData = (Write<'a, SystemApi>,);

    fn run(&mut self, mut sys: Self::SystemData) {
        let sys = &mut *sys.0;
        loop {
            let message = sys.0.poll_message().unwrap();
            match message {
                io::Message::None => break,
                io::Message::SystemInternal => continue,
                io::Message::Event(event) => match event {
                    Event::KeyDown(_, Key::ARROW_LEFT) => (),
                    Event::KeyDown(_, Key::ARROW_RIGHT) => (),
                    Event::KeyUp(_, Key::ARROW_RIGHT) => (),
                    Event::KeyUp(_, Key::ARROW_LEFT) => (),
                    Event::KeyDown(_, Key::KEY_P) => sys.0.play_sound(registry::SOUND.id),
                    Event::KeyDown(_, Key::KEY_S) => sys.0.stop_sound(registry::SOUND.id),
                    Event::KeyDown(_, Key::DIGIT1) => log::set_max_level(log::LevelFilter::Info),
                    Event::KeyDown(_, Key::DIGIT2) => log::set_max_level(log::LevelFilter::Debug),
                    Event::KeyDown(_, Key::DIGIT3) => log::set_max_level(log::LevelFilter::Trace),
                    Event::KeyDown(_, Key::ENTER) => (),
                    _ => (),
                },
            }
        }
    }
}
