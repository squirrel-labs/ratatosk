use crate::collide::Collidable;
use crate::math::Vec2;

const GRAVITY: Vec2 = Vec2::new(0.0, -9.81);

pub struct PhysicalEntity {
    pos: Vec2,
    vel: Vec2,
    accel: Vec2,
    gravity: f32,
    hitbox: Box<dyn Collidable>,
}

impl PhysicalEntity {
    /// Get the current position vector
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }

    /// Get the current mutable position vector
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }

    /// Get a multiplication factor for the gravtation acceleration
    /// 0.0 stands for no gravity, 1.0 stands for normal gravity
    pub fn gravity(&self) -> f32 {
        self.gravity
    }

    /// Get the current velocity vector
    pub fn vel(&self) -> &Vec2 {
        &self.vel
    }

    /// Set the current mutable velocity vector
    pub fn vel_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }

    /// Get the curren acceleration vector
    pub fn accel(&self) -> &Vec2 {
        &self.accel
    }

    /// Set the current acceleration vector
    pub fn accel_mut(&mut self) -> &mut Vec2 {
        &mut self.accel
    }

    pub fn hitbox(&self) -> &dyn Collidable {
        self.hitbox.as_ref()
    }

    pub fn hitbox_mut(&mut self) -> &mut dyn Collidable {
        self.hitbox.as_mut()
    }

    /// Recalculate object properties
    pub fn update(&mut self, dt: f32) {
        let accel = self.gravity * GRAVITY + self.accel;
        self.vel += dt * accel;
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
