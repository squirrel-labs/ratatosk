mod character;
mod structs;
mod textures;
use std::fs::File;
use std::io::BufReader;

use std::env;
use spine::skeleton::Skeleton;

fn usage() {
    eprintln!("Usage: bone_cake [Name_tex.json]");
    std::process::exit(1);
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Wrong number of arguments specified!");
        usage();
    }
    let mut base = args.next_back().unwrap();
    let tex = base.split_off(base.len() - 9); // 8 == "tex.json".len()
    let t = textures::load_textures(format!("{}_tex.json", base));
    println!("Textures loaded succesfully");
    let file = File::open(format!("{}.json", base).as_str()).unwrap();
    let skeleton = Skeleton::from_reader(file).unwrap();

    let skins = skeleton.get_skins_names();
    let animations = skeleton.get_animations_names();

    let animation = skeleton.get_animated_skin("default", Some("standing")).unwrap();

    for frame in animation.run(0.16) {
        for sprite in frame {
            println!("{:?}", sprite.srt.to_matrix3());
        }
    }

    //let s = structs::load_character(format!("{}ske.json", base));
    println!("{:?}", skins);
    println!("{:?}", animations);
}
