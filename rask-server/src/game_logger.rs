use colored::*;

fn color_level(level: log::Level) -> colored::ColoredString {
    let text = format!("{: <8}", level);
    match level {
        log::Level::Error => text.red().bold(),
        log::Level::Warn => text.yellow(),
        log::Level::Info => text.green(),
        log::Level::Debug => text.cyan(),
        log::Level::Trace => text.magenta(),
    }
}

pub fn init_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} > {}",
                color_level(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Off)
        .level_for("tokio_reactor", log::LevelFilter::Off)
        .level_for("reqwest", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
