use crate::math::Vec2;

const GRAVITY: Vec2 = Vec2::new(0.0, -9.81);

pub trait Move {
    fn gravity() -> bool;
    fn position(&mut self) -> &mut Vec2;
    fn velocity(&mut self) -> &mut Vec2;

    fn update(&mut self, dt: f32) {
        if Self::gravity() {
            *self.velocity() += dt * GRAVITY;
        }
        let vel = *self.velocity();
        *self.position() += dt * vel;
    }
}

pub struct World {
    players: Vec<Player>,
    entities: Vec<Entity>,
    ground: Ground,
    background: Background,
}

pub struct Player {
    position: Vec2,
    velocity: Vec2,
    skeleton: Skeleton,
}

impl Move for Player {
    fn gravity() -> bool {
        true
    }

    fn position(&mut self) -> &mut Vec2 {
        &mut self.position
    }

    fn velocity(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }
}

pub struct Entity;

pub struct Skeleton;

pub struct Ground;

pub struct Background;
