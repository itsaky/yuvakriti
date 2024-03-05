/*
 * Copyright (c) 2024 The YuvaKriti Lang Authors.
 *
 * This program is free software: you can redistribute it and/or modify it under the
 *  terms of the GNU General Public License as published by the Free Software
 *  Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this
 * program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fmt::Debug;
use colored::{ColoredString, Colorize};

use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub const LOG_LEVEL: Level = Level::Debug;
const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Debug;
static LOGGER: YKLogger = YKLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LOG_LEVEL_FILTER))
}

pub struct YKLogger;

impl Log for YKLogger {

    fn enabled(&self, metadata: &Metadata) -> bool {
        return metadata.level() < LOG_LEVEL;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut message = format_log_record(record);
            let mut colored: ColoredString;
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
    return format!("{} - {}", record.level(), record.args())
}