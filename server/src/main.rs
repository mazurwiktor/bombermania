#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate log;
extern crate simplelog;
#[macro_use] extern crate rocket;

use simplelog::{TermLogger, LevelFilter, Level, Config};

#[get("/")]
fn index() -> &'static str {
    "here we'll serve client application"
}

fn main() {
    let logger_config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Debug),
        location: Some(Level::Debug),
        time_format: Some("%T%.3f") };
    let _ = TermLogger::init(LevelFilter::Trace, logger_config);
    info!("server initialized");

    rocket::ignite().mount("/", routes![index]).launch();
}
