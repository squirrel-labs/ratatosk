mod textures;

use std::env;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!(
            "Wrong number of arguments specified!\n{}",
            "Usage: bone_cake [Name_tex.json]"
        );
        std::process::exit(1);
    }
    let t = textures::load_char(args.next_back().unwrap());
    println!("Textures loaded succesfully")
}
