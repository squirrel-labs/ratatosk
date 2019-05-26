mod client_logger;
mod site;
mod app;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

#[wasm_bindgen(start)]
pub fn entry() {
    client_logger::init_logger();

    info!("begin running wasm application");

    let app = app::App::new().unwrap();
    app.run();
}
