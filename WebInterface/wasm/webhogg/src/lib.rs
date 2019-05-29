mod client_logger;
mod webhogg_exception;
mod app;

use wasm_bindgen::prelude::*;
use app::WebhoggApplication as App;

#[macro_use]
extern crate log;

fn run_application() {
    match App::new().and_then(|mut app| app.run()) {
        Ok(_) => info!("program terminated successfully"),
        Err(e) => error!("program terminated with failure > {}", e)
    }
}

#[wasm_bindgen(start)]
pub fn entry() {
    client_logger::init_logger();

    info!("begin running wasm application");

    run_application()
}
