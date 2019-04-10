#![feature(integer_atomics)]
#![feature(arbitrary_self_types)]
#![allow(unused_variables)]

#[macro_use]
extern crate winapi;
extern crate com_impl;
extern crate divvunspell;

extern crate parking_lot;

#[macro_use]
extern crate log;
extern crate directories;
extern crate log4rs;

#[macro_use]
extern crate lazy_static;

mod util;
use crate::util::fmt_guid;

use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

mod spell_impl;
mod spellcheckprovider;

use winapi::shared::guiddef::{IsEqualGUID, REFCLSID, REFIID};
use winapi::shared::ntdef::HRESULT;
use winapi::shared::winerror::{CLASS_E_CLASSNOTAVAILABLE, S_FALSE, S_OK};
use winapi::um::winnt::PVOID;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::Interface;

use std::path::{PathBuf, Path};

use crate::spell_impl::ClassFactory::DivvunSpellCheckProviderFactoryClassFactory;
use winapi::um::unknwnbase::IClassFactory;

mod speller_cache;
mod speller_repository;
mod wordlists;

use crate::speller_repository::SpellerRepository;

lazy_static! {
    pub static ref SPELLER_REPOSITORY: SpellerRepository = {
        let mut dictionaries: Vec<String> = vec!();
        // APPDATA dictionaries
        {
            if let Some(mut path) = std::env::var("APPDATA").ok().and_then(|p| Path::new(&p).parent().map(|p| p.to_path_buf())) {
                path.push("Local");
                path.push("Programs");
                path.push("WinDivvun");
                path.push("Spellers");
                if let Some(path) = path.to_str() {
                    dictionaries.push(path.to_string());
                }
            }
        }

        // Program Files dictionaries
        {
            let path = PathBuf::from(util::get_module_path().unwrap())
                .parent()
                .unwrap()
                .to_path_buf();
            if let Some(path) = path.to_str() {
                dictionaries.push(path.to_string());
            }
        }
        
        info!("Initializing with speller repositories: {:?}", dictionaries);
        SpellerRepository::new(dictionaries)
    };
}

fn initialize_logging() -> Option<()> {
    let mut path = PathBuf::from(util::get_module_path().unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    path.push("divvunlog.txt");

    let logfile = FileAppender::builder().build(path).ok()?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(log::LevelFilter::Info),
        )
        .ok()?;

    log4rs::init_config(config).ok()?;

    Some(())
}

#[no_mangle]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    info!("DllCanUnloadNow");
    // TODO: HMMMMMMMM
    S_FALSE
}

#[no_mangle]
extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: PVOID) -> bool {
    match reason_for_call {
        DLL_PROCESS_ATTACH => {
            initialize_logging();

            info!("Library loaded! procid = {}", std::process::id());
            info!("{:?}", std::env::current_dir());
            info!("{:?}", std::env::current_exe());
        }
        DLL_PROCESS_DETACH => {
            info!("Library unloaded :(");
        }
        _ => (),
    }

    true
}

#[no_mangle]
extern "stdcall" fn DllGetClassObject(rclsid: REFCLSID, riid: REFIID, ppv: *mut PVOID) -> HRESULT {
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

    CLASS_E_CLASSNOTAVAILABLE
}
