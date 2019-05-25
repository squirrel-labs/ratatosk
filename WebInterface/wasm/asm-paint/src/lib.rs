use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = document)]
    fn write(s: &str);
}

#[wasm_bindgen(start)]
pub fn entry() {
    console_log!("hello {} wasm", 44);

    write("gooo");
}
