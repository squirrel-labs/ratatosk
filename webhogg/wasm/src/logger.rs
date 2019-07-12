use wasm_bindgen::prelude::*;
use log::Log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    pub fn __console_log_int(f: u32);

    #[wasm_bindgen(js_namespace=console, js_name=log)]
    pub fn __console_log_colored2(f: &str, c1: &str, c2: &str);
}

pub fn log_num(n: u32) { __console_log_int(n) }

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
    __console_log_colored2(&format!("{}", rec.args()),
                           &format!("color: {}", match rec.level() {
        log::Level::Trace => "violet",
        log::Level::Debug => "blue",
        log::Level::Info => "green",
        log::Level::Warn => "orange",
        log::Level::Error => "red"
    }), "");
}

pub fn init_logger() {
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
        .chain(fern::Output::call(log))
        .chain(fern::Output::call(|r|()))
        .apply().unwrap();
}
