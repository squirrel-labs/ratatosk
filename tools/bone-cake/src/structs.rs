use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct AnimateChar {
    frameRate: u8,
    name: String,
    version: String,
    compatibleVersion: String,
    armature: Vec<Armature>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct AABB {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Transform {
    x: f32,
    y: f32,
    skx: f32,
    sky: f32,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            skx: 1.0,
            sky: 1.0,
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Parent {
    name: String,
    parent: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Bone {
    name: String,
    parent: String,
    transform: Transform,
    length: f32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Slot {
    displayIndex: i32,      // `json: "displayIndex,omitempty"`
    name: String,           // `json: "name"`
    parent: String,         // `json: "parent"`
    display: Vec<Display>,  // `json: "display"`
}

#[derive(Deserialize, Debug)]
pub struct Display {
    name: String,  //`json: "name"`
}

#[derive(Deserialize, Debug)]
pub struct Skin {
    slot: Vec<Slot>,  //`json: "slot"`
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct TranslateFrame {
    tweenEasing: i32,  // `json: "tweenEasing,omitempty"`
    y: f32,            // `json: "y,omitempty"`
    x: f32,            // `json: "y,omitempty"`
    duration: i32,     // `json: "duration,omitempty"`
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct RotateFrame {
    tweenEasing: i32,  // `json: "tweenEasing,omitempty"`
    rotate: f32,       // `json: "rotate,omitempty"`
    duration: i32,     // `json: "duration,omitempty"`
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct BoneMov {
    name: String,                         // `json: "name"`
    translateFrame: Vec<TranslateFrame>,  // `json: "translateFrame,omitempty"`
    rotateFrame: Vec<RotateFrame>,        // `json: "rotateFrame,omitempty"`
}

#[derive(Deserialize, Debug)]
pub struct Animation {
    duration: i32,       // `json: "duration"`
    playTimes: i32,      // `json: "playTimes"`
    name: String,        // `json: "name"`
    bone: Vec<BoneMov>,  // `json: "bone"`
}

#[derive(Deserialize, Debug)]
pub struct DefaultActions {
    gotoAndPlay: String,  // `json: "gotoAndPlay"`
}

#[derive(Deserialize, Debug)]
pub struct Canvas {
    x: i32,       // `json: "x"`
    y: i32,       // `json: "y"`
    width: i32,   // `json: "width"`
    height: i32,  // `json: "height"`
}

#[derive(Deserialize, Debug)]
pub struct Armature {
    #[serde(rename = "type")]
    type_name: String,                    // `json: "type"`
    frameRate: i32,                       // `json: "frameRate"`
    name: String,                         // `json: "name"`
    aabb: AABB,                           // `json: "aabb"`
    bone: Vec<Bone>,                      // `json: "bone"`
    slot: Vec<Slot>,                      // `json: "slot"`
    skin: Vec<Skin>,                      // `json: "skin"`
    animation: Vec<Animation>,            // `json: "animation"`
    defaultActions: Vec<DefaultActions>,  // `json: "defaultActions"`
    canvas: Canvas,                       // `json: "canvas"`
}

pub fn load_char(path: String) -> Result<AnimateChar, Box<dyn Error>> {
    println!("Loading texture file: {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let t = serde_json::from_reader(reader)?;

    Ok(t)
}
