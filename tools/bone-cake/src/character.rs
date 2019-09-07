use std::collections::HashMap;

use rask_engine::math::Vec2;

pub struct Character<'a> {
    root: Bone,
    bones: HashMap<&'a str, Bone>,
    animations: HashMap<&'a str, Animation>,
}

struct Bone {
    pos: Vec2,
    transform: Transform,
}

struct Animation {}

struct Transform {}
