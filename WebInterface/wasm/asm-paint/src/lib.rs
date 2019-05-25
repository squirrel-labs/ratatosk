use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn entry() {
    use web_sys;
    console_log!("hello {} wasm", 42);

    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();

    let body = document.body().unwrap();

    //body.set_inner_html("<marquee><h1 style='font-size: 100px'>Hello from WASM</h1></marquee>");

    body.set_inner_html("oho");
}
