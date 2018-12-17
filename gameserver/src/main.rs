#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::{TermLogger, LevelFilter, Level, Config};

mod ecs;

fn main() {
    let logger_config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Debug),
        location: Some(Level::Debug),
        time_format: Some("%T%.3f") };
    let _ = TermLogger::init(LevelFilter::Trace, logger_config);
    info!("gameserver initialized");
}
