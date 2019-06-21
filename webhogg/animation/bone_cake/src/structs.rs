pub struct Animation {
    frameRate: u8,
    name: String,
    version: f32,
    compatibleVersion: f32,
    armature: Vec<Armature>,
}

pub struct AABB {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub struct Transform {
    x: f32,
    y: f32,
}

pub struct Parent {
    name: String,
    parent: String,
}

pub struct Bone {
    name: String,
    parent: String,
    transform: Transform,
}
pub struct Armature {
    type_name: String,
    frameRate: u8,
    aabb: AABB,
    bone: Vec<Bone>,
    slot: Vec<Parent>,

}
