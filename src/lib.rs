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
extern crate directories;

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
        SpellerRepository::new(vec![path.to_str().unwrap().to_string()])
    };
}

fn initialize_logging() -> Option<()> {
    let mut path = PathBuf::from(util::get_module_path().unwrap()).parent().unwrap().to_path_buf();
    path.push("divvunlog.txt");
    
    let logfile = FileAppender::builder()
        .build(path).ok()?;
    
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(log::LevelFilter::Info)).ok()?;
    
    log4rs::init_config(config).ok()?;

    Some(())
}

#[no_mangle]
pub extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    info!("DllCanUnloadNow");
    // TODO: HMMMMMMMM
    S_FALSE
}

#[no_mangle]
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: PVOID) -> bool {
    match reason_for_call {
        DLL_PROCESS_ATTACH => {
            initialize_logging();

            info!("Library loaded! procid = {}", std::process::id());
            info!("{:?}", std::env::current_dir());
            info!("{:?}", std::env::current_exe());
            // // info!("{:?}", known_folder(&knownfolders::FOLDERID_RoamingAppData));
            // // info!("prj {:?}", directories::ProjectDirs::from("com", "Divvun", "System Spell Checker"));
            // for (key, value) in std::env::vars() {
            //     info!("{}: {}", key, value);
            // }

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
    // let archive = SpellerArchive::new(r"C:\Program Files\SpellCheckTest\dicts\se.zhfst").unwrap();
    // let speller = archive.speller();
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
