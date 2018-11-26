#![feature(integer_atomics)]

#[macro_use]
extern crate winapi;
extern crate com_impl;
// causes unresolved references together with log4rs
// extern crate hfstospell;


#[macro_use]
extern crate log;
extern crate log4rs;
extern crate dirs;

mod util;
use util::fmtGuid;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

mod spellcheckprovider;
mod spell_impl;

use winapi::um::winnt::PVOID;
use winapi::um::winuser::MessageBoxW;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::shared::ntdef::HRESULT;
use winapi::shared::guiddef::{REFCLSID, REFIID, IsEqualGUID, GUID};
use winapi::shared::winerror::{S_OK, CLASS_E_CLASSNOTAVAILABLE};
use winapi::Interface;

use std::path::PathBuf;

use spellcheckprovider::{ISpellCheckProviderFactory};
use winapi::um::unknwnbase::{IClassFactory};
use spell_impl::ClassFactory::DivvunSpellCheckProviderFactoryClassFactory;

// mod util;

fn initialize_logging() {
    let mut path = dirs::home_dir().unwrap_or(PathBuf::from("E:\\ttc\\divvun-win-spellcheck"));
    path.push("divvunlog.txt");

    let logfile = FileAppender::builder()
        .build(path).unwrap();
    
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(log::LevelFilter::Info)).unwrap();
    
    log4rs::init_config(config);
}

fn test(msg: &str) {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
}

#[no_mangle]
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: PVOID) -> bool {
    match reason_for_call {
        DLL_PROCESS_ATTACH => {
            initialize_logging();

            info!("Library loaded!");
            info!("{:?}", dirs::desktop_dir());
        },
        DLL_PROCESS_DETACH => {
            info!("Library unloaded :(");
        },
        _ => ()
    } 

    return true;
}

#[no_mangle]
pub extern "stdcall" fn DllGetClassObject(rclsid: REFCLSID, riid: REFIID, ppv: *mut PVOID) -> HRESULT {
    unsafe {
        *ppv = std::ptr::null_mut();

        info!("DllGetClassObject");
        info!("rclsid: {}", fmtGuid(&*rclsid));
        info!("riid {}", fmtGuid(&*riid));
        info!("want {}", fmtGuid(&ISpellCheckProviderFactory::uuidof()));
        info!("want {:?}", IsEqualGUID(&ISpellCheckProviderFactory::uuidof(), &*rclsid));
    

        if IsEqualGUID(&*riid, &IClassFactory::uuidof()) {
            info!("class factory created");
            let fac = DivvunSpellCheckProviderFactoryClassFactory::new();
            *ppv = fac as PVOID;
            return S_OK;
        }
    }

    return CLASS_E_CLASSNOTAVAILABLE;
}


#[test]
fn things() {
    initialize_logging();

    info!("Library loaded!");
    test("hello world");
}