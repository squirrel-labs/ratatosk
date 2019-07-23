use textures::*;
use structs::AnimateChar;
use std::collection::HashMap;

pub struct Char {
    root: Bone,
    bones: HashMap<&str, Bone>,
    animations: HashMap<&str, Animation>,
}
