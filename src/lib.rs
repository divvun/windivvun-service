#![allow(unused_variables)]

#[macro_use]
extern crate winapi;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate lazy_static;

mod util;
use crate::util::fmt_guid;

mod spell_impl;
mod spellcheckprovider;

use winapi::shared::guiddef::{IsEqualGUID, REFCLSID, REFIID};
use winapi::shared::ntdef::HRESULT;
use winapi::shared::winerror::{CLASS_E_CLASSNOTAVAILABLE, S_FALSE, S_OK};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::winnt::PVOID;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::Interface;

use std::path::{Path, PathBuf};

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
                .parent().unwrap()
                .parent().unwrap()
                .join("Spellers");
            if let Some(path) = path.to_str() {
                dictionaries.push(path.to_string());
            }
        }

        info!("Initializing with speller repositories: {:?}", dictionaries);
        SpellerRepository::new(dictionaries)
    };
}

fn initialize_logging() -> anyhow::Result<(), ()> {
    let debug_path = PathBuf::from(util::get_module_path().unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("debug");
    if debug_path.exists() {
        unsafe { AllocConsole() };
        tracing_subscriber::fmt::fmt()
            .with_env_filter("windivvun=trace")
            .init();
        debug!("# Logging initialized");
        debug!(r"[!] To disable, delete: {} [!]", debug_path.display());
        debug!("User: {}", whoami::username());
    }
    Ok(())
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, std::io::Error> {
    use std::ffi::OsStr;
    use std::io::Error;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MessageBoxW, MB_OK};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe { MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK) };
    if ret == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret)
    }
}

#[no_mangle]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    info!("DllCanUnloadNow");
    // TODO: HMMMMMMMM
    S_FALSE
}

#[no_mangle]
extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: PVOID) -> bool {
    std::env::set_var("RUST_BACKTRACE", "full");
    match reason_for_call {
        DLL_PROCESS_ATTACH => {
            initialize_logging().unwrap();

            info!("Library loaded! procid = {}", std::process::id());
            info!("{:?}", std::env::current_dir());
            info!("{:?}", std::env::current_exe());

            // print_message("load");
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
