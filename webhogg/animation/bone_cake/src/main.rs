mod structs;
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
    let mut base = args.next_back().unwrap();
    let tex = base.split_off(base.len() - 8);
    let t = textures::load_char(format!("{}tex.json", base));
    let s = structs::load_char(format!("{}ske.json", base));
    println!("Textures loaded succesfully");
    println!("{:?}", s)
}
