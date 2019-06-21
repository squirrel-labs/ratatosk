use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn __console_log_colored2(f: &str, c1: &str, c2: &str);
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
        .apply().unwrap();
}
