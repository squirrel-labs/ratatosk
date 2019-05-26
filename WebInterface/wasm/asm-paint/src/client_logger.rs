use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=debug)]
    fn __console_debug_colored2(f: &str, c1: &str, c2: &str);
    #[wasm_bindgen(js_namespace=console, js_name=info)]
    fn __console_info_colored2(f: &str, c1: &str, c2: &str);
    #[wasm_bindgen(js_namespace=console, js_name=warn)]
    fn __console_warn_colored2(f: &str, c1: &str, c2: &str);
    #[wasm_bindgen(js_namespace=console, js_name=error)]
    fn __console_error_colored2(f: &str, c1: &str, c2: &str);
}

fn log(rec: &log::Record) {
    let log_fn = match rec.level() {
        log::Level::Trace | log::Level::Debug => __console_debug_colored2,
        log::Level::Info => __console_info_colored2,
        log::Level::Warn => __console_warn_colored2,
        log::Level::Error => __console_error_colored2,
    };
    log_fn(&format!("{}", rec.args()),
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
        .apply().unwrap();
}

