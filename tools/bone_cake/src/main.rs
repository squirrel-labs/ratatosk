mod structs;
mod textures;

use std::env;

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
    let tex = base.split_off(base.len() - 8);  // 8 == "tex.json".len()
    let t = textures::load_textures(format!("{}tex.json", base));
    let s = structs::load_char(format!("{}ske.json", base));
    println!("Textures loaded succesfully");
    println!("{:?}", s)
}
