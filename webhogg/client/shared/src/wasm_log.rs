use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(msg: &str);

    #[wasm_bindgen(js_namespace=console, js_name=log)]
    pub fn log_num(msg: u32);
}
