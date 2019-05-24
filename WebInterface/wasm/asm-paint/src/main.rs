use std::os::raw::c_int;

#[no_mangle]
pub extern fn run_infinite() {
    std::thread::spawn(move || {
        loop {
            println!("iterating infititely");
        }
    });
}

fn main() {
    println!("haha from wasm lol");
}
