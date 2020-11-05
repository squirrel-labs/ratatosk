use crate::events::Event;
use crate::events::{self, Key};
use crate::io;
use crate::math::{Mat3, Vec2};
use crate::resources::{
    self,
    registry::{self, CharacterInfo, ResourceInfo},
    GetStore,
};
use crate::EngineError;
use core::time::Duration;
use specs::WorldExt;
use specs::{prelude::*, Component};

const GRAVITY: Vec2 = Vec2::new(0.0, -9.807);

pub struct Level {}

/// An interface for the game server to interact with the game.
pub trait GameEngine {
    /// Create a new game.
    fn new(pool: std::sync::Arc<rayon::ThreadPool>, system: Box<dyn io::SystemApi>) -> Self;

    fn load_level(level: Level);

    /// Do a logic tick.
    /// May cause an `EngineError`.
    fn tick(&mut self, dt: Duration, elapsed: Duration) -> Result<(), EngineError>;
}

/// The rask specific implementation of the `GameEngine`
#[allow(dead_code)]
pub struct RaskEngine {
    world: World,
    tick_dispatcher: Dispatcher<'static, 'static>,
}

#[derive(Debug, Default)]
struct Gravitation(Vec2);

#[derive(Debug, Default)]
struct Texture_ids(Vec<u32>);

#[derive(Debug, Default)]
struct DeltaTime(std::time::Duration);

#[derive(Debug, Default)]
struct ElapsedTime(std::time::Duration);

struct SystemApi(Box<dyn io::SystemApi>);

impl Default for SystemApi {
    fn default() -> Self {
        Self(Box::new(io::DummySystemApi))
    }
}

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Vel(Vec2);

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Pos(Vec2);

#[derive(Debug, Clone, Component)]
#[storage(DenseVecStorage)]
struct Animation {
    id: u32,
    animation: String,
}

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
struct Sprite {
    id: u32,
    scale_x: f32,
    scale_y: f32,
}

#[derive(Debug, Default, Clone, Copy, Component)]
#[storage(NullStorage)]
struct Static;

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Resource(ResourceInfo);

impl Default for Resource {
    fn default() -> Self {
        Resource(resources::registry::EMPTY)
    }
}

#[derive(Debug, Clone, Copy, Component)]
#[storage(DenseVecStorage)]
struct CharacterResource(CharacterInfo);
impl Default for CharacterResource {
    fn default() -> Self {
        CharacterResource(resources::registry::CHAR)
    }
}

struct EventSystem;
struct VelocitySystem;
struct GravitationSystem;
struct RenderSystem;

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
        ReadStorage<'a, Animation>,
        Read<'a, ElapsedTime>,
        Write<'a, SystemApi>,
        Write<'a, Texture_ids>,
    );

    fn run(&mut self, (pos, sprite, anim, elapsed, mut sys, mut tex_ids): Self::SystemData) {
        let mut sprites = Vec::new();
        let res = &*resources::RESOURCE_TABLE.read();
        for (pos, sprite) in (&pos, &sprite).join() {
            if sys.0.check_fetched(sprite.id) {
                sprites.push(resources::Sprite::new(
                    Mat3::translation(pos.0.x(), pos.0.y())
                        * Mat3::scaling(sprite.scale_x, sprite.scale_y),
                    sprite.id,
                    0,
                ))
            }
        }
        for (pos, anim) in (&pos, &anim).join() {
            if sys.0.check_fetched(anim.id) {
                let trans = Mat3::translation(pos.0.x(), pos.0.y());
                let cha: Result<&Box<resources::Character>, EngineError> =
                    res.get(anim.id as usize);
                let cha = cha.unwrap();
                for sp in cha
                    .interpolate(elapsed.0.as_secs_f32(), anim.animation.as_str())
                    .unwrap()
                {
                    let sp = sp.unwrap();
                    sprites.push(resources::Sprite::new(
                        trans * sp.transform,
                        anim.id,
                        sp.att_id,
                    ))
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

    fn run(&mut self, (mut sys): Self::SystemData) {
        let mut sys = &mut *sys.0;
        loop {
            let message = sys.0.poll_message().unwrap();
            //log::info!("event: {:?}", message);
            match message {
                io::Message::None => break,
                io::Message::SystemInternal => continue,
                io::Message::Event(event) => {
                    log::info!("event: {:?}", event);
                    match event {
                        Event::KeyDown(_, Key::ARROW_LEFT) => (),
                        Event::KeyDown(_, Key::ARROW_RIGHT) => (),
                        Event::KeyUp(_, Key::ARROW_RIGHT) => (),
                        Event::KeyUp(_, Key::ARROW_LEFT) => (),
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
                        Event::KeyDown(_, Key::ENTER) => (),
                        _ => (),
                    }
                }
            }
        }
    }
}
impl GameEngine for RaskEngine {
    fn new(pool: std::sync::Arc<rayon::ThreadPool>, mut system: Box<dyn io::SystemApi>) -> Self {
        system.fetch_resource(registry::EMPTY);
        system.fetch_resource(registry::SOUND);
        system.fetch_character_resource(registry::CHAR);

        let mut world: specs::World = specs::WorldExt::new();
        world.insert(Gravitation(GRAVITY));
        world.insert(DeltaTime(Duration::from_millis(10)));
        world.insert(ElapsedTime(Duration::from_millis(0)));
        world.insert(Texture_ids(Vec::new()));
        world.insert(SystemApi(system));

        let mut tick_dispatcher = DispatcherBuilder::new()
            .with_pool(pool)
            .with(EventSystem, "events", &[])
            .with(GravitationSystem, "gravitation", &["events"])
            .with(VelocitySystem, "velocity", &["gravitation"])
            .with_barrier()
            .with(RenderSystem, "rendering", &[])
            .build();

        tick_dispatcher.setup(&mut world);
        let backround = world
            .create_entity()
            .with(Pos(Vec2::new(0.0, 0.0)))
            .with(Sprite {
                id: registry::EMPTY.id,
                scale_x: 1.0,
                scale_y: 1.0,
            })
            .with(Static)
            .build();
        let char = world
            .create_entity()
            .with(Pos(Vec2::new(0.0, -0.8)))
            .with(Animation {
                id: registry::CHAR.id,
                animation: "walking".to_string(),
            })
            .with(Static)
            .build();
        Self {
            world,
            tick_dispatcher,
        }
    }

    fn load_level(_level: Level) {}

    fn tick(&mut self, dt: Duration, elapsed: Duration) -> Result<(), EngineError> {
        *self.world.write_resource::<DeltaTime>() = DeltaTime(dt);
        *self.world.write_resource::<ElapsedTime>() = ElapsedTime(elapsed);
        self.tick_dispatcher.dispatch_seq(&mut self.world);
        self.world.maintain();
        Ok(())
    }
}
