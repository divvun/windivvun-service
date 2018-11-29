#![feature(integer_atomics)]
#![feature(duration_as_u128)]
#![feature(arbitrary_self_types)]
#![allow(unused_variables)]

#[macro_use]
extern crate winapi;
extern crate com_impl;
extern crate hfstospell;
extern crate glob;

extern crate parking_lot;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate dirs;

#[macro_use]
extern crate lazy_static;

mod util;
use util::fmt_guid;

use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

mod spellcheckprovider;
mod spell_impl;

use winapi::um::winnt::PVOID;
use winapi::um::winuser::MessageBoxW;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::shared::ntdef::HRESULT;
use winapi::shared::guiddef::{REFCLSID, REFIID, IsEqualGUID, GUID};
use winapi::shared::winerror::{S_OK, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};
use winapi::Interface;

use std::path::PathBuf;

use spellcheckprovider::{ISpellCheckProviderFactory};
use winapi::um::unknwnbase::{IClassFactory};
use spell_impl::ClassFactory::DivvunSpellCheckProviderFactoryClassFactory;

mod speller_repository;
mod speller_cache;
mod wordlists;

use speller_repository::SpellerRepository;

use std::panic;

lazy_static! {
    pub static ref SPELLER_REPOSITORY: SpellerRepository = {
        let mut path = PathBuf::from(util::get_module_path().unwrap()).parent().unwrap().to_path_buf();
        path.push("dicts");
        SpellerRepository::new(path.to_str().unwrap())
    };
}

fn initialize_logging() {
    let mut path = PathBuf::from(util::get_module_path().unwrap()).parent().unwrap().to_path_buf();
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
pub extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    info!("DllCanUnloadNow");
    // TODO: HMMMMMMMM
    S_FALSE
    //S_OK
}

#[no_mangle]
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: PVOID) -> bool {
    match reason_for_call {
        DLL_PROCESS_ATTACH => {
            panic::set_hook(Box::new(|_| {
                info!("Custom panic hook");
            }));

            initialize_logging();

            info!("Library loaded! procid = {}", std::process::id());
            info!("{:?}", std::env::current_dir());
            info!("{:?}", std::env::current_exe());
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

        info!("rclsid: {}", fmt_guid(&*rclsid));
        info!("riid {}", fmt_guid(&*riid));
        if IsEqualGUID(&*riid, &IClassFactory::uuidof()) {
            let fac = DivvunSpellCheckProviderFactoryClassFactory::new();
            *ppv = fac as PVOID;
            info!("class factory created");
            return S_OK;
        }

        error!("Invalid interface requested");
    }

    return CLASS_E_CLASSNOTAVAILABLE;
}

#[test]
fn things() {
    //test("hello world");
    use hfstospell::archive::SpellerArchive;
    let archive = SpellerArchive::new(r"C:\Program Files\SpellCheckTest\dicts\se.zhfst").unwrap();
    let speller = archive.speller();
    //println!("a {:?}", speller.clone().is_correct("heallu"));
    //println!("b {:?}", speller.clone().suggest("heallu"));
}

#[test]
fn name_resolve() {
    let tag = util::resolve_locale_name("en");
    println!("res {:?}", tag);
    let tag = util::resolve_locale_name("smj");
    println!("res {:?}", tag);

    println!("{:?}", SPELLER_REPOSITORY.get_speller_archives());

    println!("{:?}", SPELLER_REPOSITORY.get_supported_languages());
    //rep.add_dictionary("sv-SE", r"C:\Program Files\SpellCheckTest\dicts\sme.zhfst");
}
