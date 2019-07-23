use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=debug)]
    fn _log_debug(msg: &str, color1: &str, color2: &str);

    #[wasm_bindgen(js_namespace=console, js_name=info)]
    fn _log_info(msg: &str, color1: &str, color2: &str);

    #[wasm_bindgen(js_namespace=console, js_name=warn)]
    fn _log_warn(msg: &str, color1: &str, color2: &str);

    #[wasm_bindgen(js_namespace=console, js_name=error)]
    fn _log_error(msg: &str, color1: &str, color2: &str);
}

pub struct WasmLog();

impl WasmLog {
    pub fn new() -> Self { WasmLog() }
}

impl log::Log for WasmLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        let (log, name, color): (fn(&str, &str, &str), &str, &str) =
                         match record.level() {
            log::Level::Trace => (_log_debug, "trace", "color:plum;"),
            log::Level::Debug => (_log_debug, "debug", "color:indigo;"),
            log::Level::Info => (_log_info, "info", "color:forestgreen;"),
            log::Level::Warn => (_log_warn, "warn", "color:orangered;"),
            log::Level::Error => (_log_error, "error", "color:firebrick;"),
        };
        let msg = &format!("{}", format_args!("%c{}%c\t{}", name, record.args()));
        log(msg, color, "");
    }

    fn flush(&self) {}
}
