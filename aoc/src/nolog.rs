use log::SetLoggerError;
use log::{LevelFilter, Metadata, Record};

struct NoLogging;

impl log::Log for NoLogging {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        dbg!(_metadata);
        true
    }

    fn log(&self, record: &Record) {
        println!("{}", record.args());
    }

    fn flush(&self) {}
}

static LOGGER: NoLogging = NoLogging;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
