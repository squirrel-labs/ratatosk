#[cfg(target_arch = "wasm32")]
extern "C" {
    fn log_debug(msg: *const u8, len: i32);
    fn log_info(msg: *const u8, len: i32);
    fn log_warn(msg: *const u8, len: i32);
    fn log_error(msg: *const u8, len: i32);
    fn log_panic(msg: *const u8, len: i32, file: *const u8, flen: i32, line: i32, column: i32);
}

#[derive(Default)]
pub struct WasmLog;

impl log::Log for WasmLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    #[cfg(target_arch = "wasm32")]
    fn log(&self, record: &log::Record) {
        let (log, name): (unsafe extern "C" fn(*const u8, i32), &str) = match record.level() {
            log::Level::Trace => (log_debug, "trace"),
            log::Level::Debug => (log_debug, "debug"),
            log::Level::Warn => (log_warn, "warn"),
            log::Level::Info => (log_info, "info"),
            log::Level::Error => (log_error, "error"),
        };
        let msg = &format!("{}", format_args!("%c{}%c\t{}", name, record.args()));
        unsafe { log(msg.as_ptr(), msg.len() as i32) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn log(&self, record: &log::Record) {
        println!("{}", format_args!("{}", record.args()));
    }

    fn flush(&self) {}
}

#[cfg(target_arch = "wasm32")]
pub fn init_panic_handler() {
    std::panic::set_hook(Box::new(|info| {
        let (file, line, column) = if let Some(loc) = info.location() {
            (loc.file(), loc.line() as i32, loc.column() as i32)
        } else {
            ("<unknown>", 0, 0)
        };
        if let Some(payload) = info.payload().downcast_ref::<&str>() {
            unsafe {
                log_panic(
                    payload.as_ptr(),
                    payload.len() as i32,
                    file.as_ptr(),
                    file.len() as i32,
                    line,
                    column,
                )
            }
        } else if let Some(message) = info.message() {
            let msg = format!("{}", message);
            unsafe {
                log_panic(
                    msg.as_ptr(),
                    msg.len() as i32,
                    file.as_ptr(),
                    file.len() as i32,
                    line,
                    column,
                )
            }
        }
    }));
}
