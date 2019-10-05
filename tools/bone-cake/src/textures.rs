use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct Textures {
    pub width: i64,
    #[serde(rename = "SubTexture")]
    pub sub_textures: Vec<SubTexture>,
    pub height: i64,
    pub name: String,
    #[serde(rename = "imagePath")]
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTexture {
    pub width: i64,
    pub y: i64,
    pub height: i64,
    pub name: String,
    pub x: i64,
}


pub fn load_textures(path: String) -> Result<Textures, Box<dyn Error>> {
    println!("Loading texture file: {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let t = serde_json::from_reader(reader)?;

    Ok(t)
}
