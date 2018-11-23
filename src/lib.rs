

#[macro_use]
extern crate winapi;
// causes unresolved references together with log4rs
// extern crate hfstospell;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate dirs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

mod spellcheckprovider;

use winapi::um::winnt::PVOID;


// mod impl;
// mod util;

fn initialize_logging() {
    let mut home = dirs::home_dir().unwrap();
    home.push("divvunlog.txt");

    let logfile = FileAppender::builder()
        .build(home.clone()).unwrap();
    
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(log::LevelFilter::Info)).unwrap();
    
    log4rs::init_config(config);
}

#[no_mangle]
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: PVOID) {
	initialize_logging();

    info!("Library loaded!");
}
