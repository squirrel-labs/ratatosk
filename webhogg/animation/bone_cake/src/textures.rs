use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Textures {
    name: String,
    width: f32,
    height: f32,
    imagePath: String,
    SubTexture: Vec<Texture>,
}

#[derive(Deserialize, Debug)]
pub struct Texture {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    name: String,
}

pub fn load_textures(path: String) -> Result<Textures, Box<dyn Error>> {
    println!("Loading texture file: {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let t = serde_json::from_reader(reader)?;

    Ok(t)
}
