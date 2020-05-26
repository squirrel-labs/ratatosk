use crate::collide::Collidable;
use crate::math::Vec2;

const GRAVITY: Vec2 = Vec2::new(0.0, -9.81);

pub struct PhysicalEntity {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    gravity: f32,
    hit_box: Box<dyn Collidable>,
}

impl PhysicalEntity {
    /// Get the current position vector.
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Get the current mutable position vector.
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }

    /// Get a multiplication factor for the gravitation acceleration.
    /// 0.0 stands for no gravity, 1.0 stands for normal gravity.
    pub fn gravity(&self) -> f32 {
        self.gravity
    }

    /// Get the current velocity vector.
    pub fn vel(&self) -> &Vec2 {
        &self.vel
    }

    /// Set the current mutable velocity vector.
    pub fn vel_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }

    /// Get the current acceleration vector.
    pub fn acc(&self) -> &Vec2 {
        &self.acc
    }

    /// Set the current acceleration vector.
    pub fn acc_mut(&mut self) -> &mut Vec2 {
        &mut self.acc
    }

    pub fn hit_box(&self) -> &dyn Collidable {
        self.hit_box.as_ref()
    }

    pub fn hit_box_mut(&mut self) -> &mut dyn Collidable {
        self.hit_box.as_mut()
    }

    /// Recalculate object properties.
    pub fn update(&mut self, dt: f32) {
        let acc = self.gravity * GRAVITY + self.acc;
        self.vel += dt * acc;
        self.pos += dt * self.vel;
    }
}

#[allow(dead_code)]
pub struct World {
    physical_entities: Vec<PhysicalEntity>,
    players: Vec<Player>,
    ground: Ground,
    background: Background,
}

impl World {
    // TODO: pub fn new() -> Self {  }
}

#[allow(dead_code)]
pub struct Player {
    skeleton: Skeleton,
}

pub struct Skeleton;

pub struct Ground;

pub struct Background;
