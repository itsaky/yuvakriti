use colored::ColoredString;
use colored::Colorize;
use log::Level;
use log::LevelFilter;
use log::Log;
use log::Metadata;
use log::Record;
use log::SetLoggerError;

pub(crate) const LOG_LEVEL: Level = Level::Debug;
const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Debug;
static LOGGER: YKLogger = YKLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LOG_LEVEL_FILTER))
}

pub struct YKLogger;

impl Log for YKLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        return metadata.level() < LOG_LEVEL;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format_log_record(record);
            let colored: ColoredString;
            match record.level() {
                Level::Error => colored = message.truecolor(244, 67, 54),
                Level::Warn => colored = message.truecolor(255, 171, 0),
                Level::Info => colored = message.truecolor(76, 175, 80),
                Level::Debug => colored = message.truecolor(33, 33, 33),
                Level::Trace => colored = message.truecolor(2, 136, 209),
            }

            println!("{:?}", colored);
        }
    }

    fn flush(&self) {
        todo!()
    }
}

fn format_log_record(record: &Record) -> String {
    return format!("{} - {}", record.level(), record.args());
}
