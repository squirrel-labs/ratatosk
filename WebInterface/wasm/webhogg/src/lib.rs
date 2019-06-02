mod client_logger;
mod webhogg_exception;
mod page;
mod canvas;
mod app;

use wasm_bindgen::prelude::*;
use app::WebhoggApplication as App;
use web_sys::Worker;

#[macro_use]
extern crate log;

fn run_application() {
    match App::new().and_then(|app| app.run()) {
        Ok(_) => info!("program terminated successfully"),
        Err(e) => error!("program terminated with failure > {}", e)
    }
}

#[wasm_bindgen]
pub fn game_logic_entry(worker: web_sys::Worker) {
    client_logger::init_logger();

    info!("hello from game logic wasm");
    info!("begin long calculation in game logic thread");
    worker.post_message(&wasm_bindgen::JsValue::from_str("premsg frm wasm_gLe"))
        .unwrap();
    info!("killed game logic");
}

#[wasm_bindgen]
pub fn graphics_entry(worker: web_sys::DedicatedWorkerGlobalScope,
                      canvas: web_sys::OffscreenCanvas) {
    client_logger::init_logger();

    info!("hello from graphics wasm {:?}", canvas);
    let handler = wasm_bindgen::closure::Closure::once_into_js(
                    (|e: web_sys::MessageEvent| {
        info!("things are getting wired: {:?}", e.data());
    }));

    worker.set_onmessage(Some(&js_sys::Function::from(handler)));
    entry2();
    info!("killed graphics");
}

pub fn entry2() {
    // client_logger::init_logger();

    info!("begin running wasm application");

    run_application()
}
