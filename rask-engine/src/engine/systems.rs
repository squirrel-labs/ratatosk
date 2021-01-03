use super::components::*;

use crate::boxes::RBox;
use crate::collide::{Collidable, Collide};
use crate::events::{Event, Key, Keyboard};
use crate::io;
use crate::math::{Mat3, Vec2};
use crate::resources::{self, registry, GetStore};
use crate::EngineError;
use specs::join::JoinIter;
use specs::prelude::*;
use specs_hierarchy::Hierarchy;

lazy_static::lazy_static! {
    pub static ref KEYBOARD: Keyboard = Keyboard::new();
}

pub struct EventSystem;
pub struct SimpleVelocitySystem;
pub struct VelocitySystem;
pub struct GravitationSystem;
pub struct RenderSystem;
pub struct MovementSystem;
pub struct CheckPresentSystem;
pub struct UpdateAnimationSystem;

impl<'a> System<'a> for SimpleVelocitySystem {
    type SystemData = (
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Mass>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut pos, vel, mass, dt): Self::SystemData) {
        for (vel, pos, _) in (&vel, &mut pos, !&mass).join() {
            pos.0 += vel.0 * dt.0.as_secs_f32();
        }
    }
}

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SubCollider>,
        ReadStorage<'a, Terrain>,
        Entities<'a>,
        ReadExpect<'a, Hierarchy<Parent>>,
        Read<'a, DeltaTime>,
    );

    fn run(
        &mut self,
        (mut pos, mut vel, mass, collider, mut transform, sub, terrain, entities, hierarchy, dt): Self::SystemData,
    ) {
        let reset_values: Vec<_> = (&collider, &vel, !&terrain, &pos, &mass, &entities)
            .par_join()
            .map(|(_col1, vel, _, _pos1, _mass, entity1)| {
                let mut percent = -1.0;
                let mut ids = (entity1.id(), 0);
                let v = vel.0 * dt.0.as_secs_f32();
                'b: for (_, _, _pos2, entity2) in (&collider, &terrain, &pos, &entities).join() {
                    for e1 in hierarchy.children(entity1) {
                        for e2 in hierarchy.children(entity2) {
                            if let (
                                Some(SubCollider { collider: c1 }),
                                Some(SubCollider { collider: c2 }),
                            ) = (sub.get(*e1), sub.get(*e2))
                            {
                                if let Some(move_out) = c1.collide_after(c2, v) {
                                    if move_out > percent {
                                        percent = move_out;
                                        ids = (entity1.id(), entity2.id());
                                    }
                                }
                            }
                        }
                    }
                }
                if percent == -1.0 {
                    (ids.0, core::u32::MAX, v)
                } else {
                    (ids.0, ids.1, v * (1.0 - percent))
                }
            })
            .collect();
        for (e1, e2, rv) in reset_values {
            let e1 = entities.entity(e1);
            if let Some(pos) = pos.get_mut(e1) {
                pos.0 += rv;
            }
            if let Some(vel) = vel.get_mut(e1) {
                if e2 != core::u32::MAX {
                    vel.0 = Vec2::zero();
                }
            }
            for e in hierarchy.children(e1) {
                if let Some(trans) = transform.get_mut(*e) {
                    trans.mat3 *= Mat3::translation(rv.x(), rv.y()); // * trans.mat3;
                }
                if let Some(sub) = sub.get(*e) {
                    //sub. += Mat3::translation(rv.x(), rv.y()) * trans.mat3;
                }
            }
        }
    }
}

impl<'a> System<'a> for GravitationSystem {
    type SystemData = (
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Present>, // TODO: Remove as it is only used for debugging
        Read<'a, Gravitation>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut vel, mass, present, g, dt): Self::SystemData) {
        let dt = dt.0.as_secs_f32();
        if dt < 0.5 {
            for (vel, _, _) in (&mut vel, &mass, &present).join() {
                vel.0 += g.0 * dt;
            }
        }
    }
}

impl<'a> System<'a> for UpdateAnimationSystem {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, SubCollider>,
        WriteStorage<'a, Vulnerable>,
        WriteStorage<'a, Damaging>,
        WriteStorage<'a, Parent>,
        ReadStorage<'a, Present>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, Pos>,
        Entities<'a>,
        ReadExpect<'a, Hierarchy<Parent>>,
        Read<'a, ElapsedTime>,
    );

    fn run(
        &mut self,
        (
            mut animations,
            mut sprite,
            mut mat3,
            collider,
            mut sub,
            mut vul,
            mut dmg,
            mut parent,
            present,
            scale,
            pos,
            entities,
            hierarchy,
            elapsed,
        ): Self::SystemData,
    ) {
        let res = &mut *resources::RESOURCE_TABLE.write();
        for (mut animation, collider, scale, pos, entity, _) in (
            &mut animations,
            &collider,
            &scale,
            &pos,
            &entities,
            &present,
        )
            .join()
        {
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

                let sprites = cha
                    .interpolate(elapsed.0.as_secs_f32() - animation.start)
                    .unwrap();
                let trans = Mat3::translation(pos.0.x(), pos.0.y());
                let scale = Mat3::scaling(scale.0.x(), scale.0.y());
                let ci = hierarchy.children(entity);
                let mut ci: Vec<_> = ci.to_vec();
                ci.sort_unstable_by_key(|x| x.id()); //TODO we might be able to remove this
                let mut ci = ci.iter();
                for (i, s) in sprites.enumerate() {
                    let s = s.unwrap();
                    let n_transform = trans * scale * s.transform;
                    let (new_mat3, new_sprite, new_sub) = (
                        Transform { mat3: n_transform },
                        Sprite {
                            id: animation.id,
                            sub_id: s.att_id,
                        },
                        SubCollider {
                            collider: Collidable::RBox(RBox::from(&n_transform)),
                        },
                    );
                    if let Some(c) = ci.next().cloned() {
                        if let Some((sprite, mat3, sub)) = JoinIter::get(
                            &mut (&mut sprite, &mut mat3, &mut sub).join(),
                            c,
                            &entities,
                        ) {
                            *mat3 = new_mat3;
                            *sprite = new_sprite;
                            *sub = new_sub;
                        }
                    } else {
                        let e = entities.create();
                        entities
                            .build_entity()
                            .with(new_mat3, &mut mat3)
                            .with(new_sub, &mut sub)
                            .with(new_sprite, &mut sprite)
                            .with(Parent { entity }, &mut parent)
                            .build();

                        match collider.mapping.get(&(i as u32)) {
                            Some(HitboxType::Damaging) => {
                                dmg.insert(e, Damaging { damage: 0.0 }).unwrap();
                            }
                            Some(HitboxType::Vulnerable) => {
                                vul.insert(e, Vulnerable { armor: 0.0 }).unwrap();
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, Present>,
        ReadStorage<'a, Transform>,
        Write<'a, SystemApi>,
        Write<'a, TextureIds>,
    );

    fn run(
        &mut self,
        (pos, sprite, scale, present, transform, mut sys, mut tex_ids): Self::SystemData,
    ) {
        let mut sprites = Vec::new();
        for (pos, sprite, scale, _) in (&pos, &sprite, &scale, &present).join() {
            sprites.push(resources::Sprite::new(
                Mat3::translation(pos.0.x(), pos.0.y()) * Mat3::scaling(scale.0.x(), scale.0.y()),
                sprite.id,
                sprite.sub_id,
            ))
        }
        for (sprite, transform, _) in (&sprite, &transform, &present).join() {
            sprites.push(resources::Sprite::new(
                transform.mat3,
                sprite.id,
                sprite.sub_id,
            ))
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
                scale.0 = Vec2::new(scale.0.x().abs(), scale.0.y());
                vel.0 = Vec2::new(speed.0, 0.0);
                "walking".to_owned()
            } else if KEYBOARD.get(Key::ARROW_LEFT) {
                scale.0 = Vec2::new(-scale.0.y().abs(), scale.0.y());
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
        Entities<'a>,
        WriteStorage<'a, Present>,
    );

    fn run(&mut self, (anim, sprite, entities, mut present): Self::SystemData) {
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
        for item in modified {
            let _ = present
                .insert(item, Present)
                .map_err(|e| log::debug!("{}", e));
        }
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
                io::Message::Event(event) => {
                    log::trace!("event: {:?}", event);
                    match event {
                        Event::KeyDown(_, Key::KEY_P) => sys.0.play_sound(registry::SOUND.id),
                        Event::KeyDown(_, Key::KEY_S) => sys.0.stop_sound(registry::SOUND.id),
                        Event::KeyDown(_, Key::DIGIT1) => {
                            log::set_max_level(log::LevelFilter::Info)
                        }
                        Event::KeyDown(_, Key::DIGIT2) => {
                            log::set_max_level(log::LevelFilter::Debug)
                        }
                        Event::KeyDown(_, Key::DIGIT3) => {
                            log::set_max_level(log::LevelFilter::Trace)
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
