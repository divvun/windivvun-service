#![cfg(windows)] 
#![allow(non_snake_case)]
#![allow(unused_variables)]

use winapi::um::winnt::HRESULT;

use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_NOINTERFACE};
use winapi::shared::guiddef::{IsEqualGUID, GUID, REFIID};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, TRUE};
use winapi::Interface;

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl, IClassFactory, IClassFactoryVtbl};
use spellcheckprovider::ISpellCheckProviderFactory;

use std::sync::atomic::{AtomicU32, Ordering};

use com_impl::{ComInterface, interface, implementation};

use ::util::fmt_guid;

use super::SpellCheckProviderFactory::DivvunSpellCheckProviderFactory;

#[interface(IClassFactory)]
pub struct DivvunSpellCheckProviderFactoryClassFactory {
    refs: AtomicU32
}

IMPL_UNKNOWN!(IClassFactory, DivvunSpellCheckProviderFactoryClassFactory);

#[implementation(IClassFactory)]
impl DivvunSpellCheckProviderFactoryClassFactory {
    fn CreateInstance(&mut self, pUnkOuter: *mut IUnknown, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
        unsafe {
            info!("CreateInstance for {}", fmt_guid(&*riid));
            if IsEqualGUID(&*riid, &ISpellCheckProviderFactory::uuidof()) {
                info!("Creating SpellCheckProviderFactory");
                let ptr = DivvunSpellCheckProviderFactory::new();
                *ppvObject = ptr as *mut _;
                return S_OK;
            }
        }
        E_NOINTERFACE
    }

    fn LockServer(&mut self, fLock: BOOL) -> HRESULT {
        info!("LockServer");
        if fLock == TRUE {
            self.AddRef();
        } else {
            self.Release();
        }

        S_OK
    }
}

impl DivvunSpellCheckProviderFactoryClassFactory {
    pub fn new() -> *mut DivvunSpellCheckProviderFactoryClassFactory {
        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1)
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}