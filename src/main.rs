

#[macro_use]
extern crate winapi;
extern crate hfstospell;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate dirs;

//use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

mod spellcheckprovider;

use winapi::um::shlobj::{SHGetKnownFolderPath, KF_FLAG_DEFAULT};
use winapi::um::shtypes::{REFKNOWNFOLDERID};
use winapi::um::combaseapi::{CoTaskMemFree};
use winapi::shared::winerror::{S_OK};
use winapi::shared::ntdef::{PWSTR};
use winapi::um::knownfolders::FOLDERID_Desktop;

use std::ptr;

// mod impl;
// mod util;

fn main() {
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

    info!("Hello, world!");

    println!("Hello, world!");
    // unsafe {
    //     let mut path: PWSTR = ptr::null_mut();
    //     if SHGetKnownFolderPath(&FOLDERID_Desktop, KF_FLAG_DEFAULT, ptr::null_mut(), &mut path) != S_OK {
    //         println!("error");
    //     } else {
    //         println!("ok");
    //         let path = String::from_utf16_lossy(&*path);
    //         println!("path: {}", path);
    //         CoTaskMemFree(path);
    //     }
    // }
    println!("{}", home.display());
    // let zhfst = hfstospell::SpellerArchive::new("./se-store.zhfst");
    // let two = zhfst.speller();
    // let res = two.suggest("nuvviDspeller");

}
