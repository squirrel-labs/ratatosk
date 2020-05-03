fn main() {
    match std::env::var("MEMORY_LAYOUT_PARAMETERS") {
        Ok(param) => print!("{}", param),
        Err(_) => {
            if cfg!(assert_memory) {
                panic!("error: could not get or parse environment variable \"MEMORY_LAYOUT_PARAMETERS\"");
            }
        }
    }
}
