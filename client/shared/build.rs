fn main() {
    println!("{}", std::env::var("MEMORY_LAYOUT_PARAMETERS").expect("error: could not get or parse environment variable \"MEMORY_LAYOUT_PARAMETERS\""));
}
