mod client_logger;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

#[wasm_bindgen(start)]
pub fn entry() {
    client_logger::init_logger();

    info!("{}", 42);
}
