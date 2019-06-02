mod client_logger;
mod webhogg_exception;
mod canvas;

use wasm_bindgen::prelude::*;
use web_sys::Worker;
use web_sys::OffscreenCanvas as ECanvas;

#[macro_use]
extern crate log;

#[wasm_bindgen]
pub fn game_logic_entry(worker: web_sys::Worker) {
    client_logger::init_logger();

    info!("hello from game logic wasm");
    worker.post_message(&wasm_bindgen::JsValue::from_str("premsg frm wasm_gLe"))
        .unwrap();
    info!("game logic terminated");
}

#[wasm_bindgen]
pub fn graphics_entry(worker: web_sys::DedicatedWorkerGlobalScope,
                      ecanvas: JsValue) {
    client_logger::init_logger();

    let ecanvas: ECanvas = js_sys::Reflect::get(&ecanvas,
                      &wasm_bindgen::JsValue::from_str("canvas"))
                        .map_err(|e| error!("could not load canvas"))
                        .unwrap().into();

    info!("hello from graphics wasm {:?}", ecanvas);
    let handler = wasm_bindgen::closure::Closure::once_into_js(
                    (|e: web_sys::MessageEvent| {
        info!("things are getting wired: {:?}", e.data());
    }));
    worker.set_onmessage(Some(&js_sys::Function::from(handler)));

    let canvas = canvas::Canvas::from_existing(&ecanvas)
        .map_err(|e| error!("{}", e))
        .unwrap();

    info!("graphics terminated");
}
