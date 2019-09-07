use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct AnimateChar {
    #[serde(rename = "frameRate")]
    pub frame_rate: Option<i64>,
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "compatibleVersion")]
    pub compatible_version: Option<String>,
    pub armature: Option<Vec<Armature>>,
}

#[derive(Debug, Deserialize)]
pub struct Armature {
    #[serde(rename = "type")]
    pub armature_type: Option<String>,
    #[serde(rename = "frameRate")]
    pub frame_rate: Option<i64>,
    pub name: Option<String>,
    pub aabb: Option<Aabb>,
    pub bone: Option<Vec<ArmatureBone>>,
    pub slot: Option<Vec<ArmatureSlot>>,
    pub skin: Option<Vec<Skin>>,
    pub animation: Option<Vec<Animation>>,
    #[serde(rename = "defaultActions")]
    pub default_actions: Option<Vec<DefaultAction>>,
}

#[derive(Debug, Deserialize)]
pub struct Aabb {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Animation {
    pub duration: Option<i64>,
    #[serde(rename = "playTimes")]
    pub play_times: Option<i64>,
    pub name: Option<String>,
    pub bone: Option<Vec<AnimationBone>>,
}

#[derive(Debug, Deserialize)]
pub struct AnimationBone {
    pub name: Option<String>,
    #[serde(rename = "translateFrame")]
    pub translate_frame: Option<Vec<TranslateFrame>>,
    #[serde(rename = "rotateFrame")]
    pub rotate_frame: Option<Vec<RotateFrame>>,
}

#[derive(Debug, Deserialize)]
pub struct RotateFrame {
    pub duration: Option<i64>,
    #[serde(rename = "tweenEasing")]
    pub tween_easing: Option<i64>,
    pub rotate: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct TranslateFrame {
    #[serde(rename = "tweenEasing")]
    pub tween_easing: Option<i64>,
    pub y: Option<f64>,
    pub duration: Option<i64>,
    pub x: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ArmatureBone {
    pub name: Option<String>,
    pub transform: Option<Transform>,
    pub parent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Transform {
    pub x: Option<f64>,
    pub y: Option<f64>,
    #[serde(rename = "skX")]
    pub sk_x: Option<f64>,
    #[serde(rename = "skY")]
    pub sk_y: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct DefaultAction {
    #[serde(rename = "gotoAndPlay")]
    pub goto_and_play: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Skin {
    pub slot: Option<Vec<SkinSlot>>,
}

#[derive(Debug, Deserialize)]
pub struct SkinSlot {
    pub name: Option<String>,
    pub display: Option<Vec<Display>>,
}

#[derive(Debug, Deserialize)]
pub struct Display {
    pub name: Option<String>,
    pub transform: Option<Transform>,
}

#[derive(Debug, Deserialize)]
pub struct ArmatureSlot {
    pub name: Option<String>,
    pub parent: Option<String>,
}

pub fn load_character(path: String) -> Result<AnimateChar, Box<dyn Error>> {
    println!("Loading texture file: {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let t = serde_json::from_reader(reader)?;

    Ok(t)
}
