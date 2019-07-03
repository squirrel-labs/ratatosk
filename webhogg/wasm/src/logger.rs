use wasm_bindgen::prelude::*;
use log::Log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    pub fn __console_log(f: &str);

    #[wasm_bindgen(js_namespace=console, js_name=log)]
    pub fn __console_log_colored2(f: &str, c1: &str, c2: &str);
}

struct WasmLogger;

impl log::Log for WasmLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            log(record)
        }
    }

    fn flush(&self) {}
}

fn log(rec: &log::Record) {
    __console_log_colored2(&format!("%c{}%c {} > {}", rec.level(), rec.target(), rec.args()),
                           &format!("color: {}", match rec.level() {
        log::Level::Trace => "violet",
        log::Level::Debug => "blue",
        log::Level::Info => "green",
        log::Level::Warn => "orange",
        log::Level::Error => "red"
    }), "");

    /*web_sys::console::log_3(&JsValue::from_str(&format!("{}", rec.args())),
                            &JsValue::from_str(&format!("color: {}", match rec.level() {
        log::Level::Trace => "violet",
        log::Level::Debug => "blue",
        log::Level::Info => "green",
        log::Level::Warn => "orange",
        log::Level::Error => "red"
    })), &JsValue::from_str(""));*/
}

//const ADDRESS: u64 = 640000;

pub fn init_logger() -> Result<(), log::SetLoggerError> {
    unsafe {
        log::set_logger(Box::leak(Box::from_raw(640000 as *mut WasmLogger)))
            .map(|()| log::set_max_level(log::LevelFilter::Info))
    }
}

/*pub fn init_logger() {
    fern::Dispatch::new().format(|out, message, record|{
        out.finish(format_args!(
                "%c{}%c {} > {}",
                record.level(),
                record.target(),
                message
                )
            )
        })
        .level(log::LevelFilter::Debug)
        //.chain(fern::Output::call(log))
        .chain(fern::Output::call(|r|()))
        .apply().unwrap();
}*/
