#![cfg(windows)] 

use winapi::um::objidlbase::IEnumString;
use winapi::um::winnt::{LPCWSTR, HRESULT};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_INVALIDARG, E_POINTER};
use winapi::shared::guiddef::{IsEqualGUID, GUID};
use winapi::shared::minwindef::{TRUE, FALSE};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::sync::atomic::{AtomicU32, Ordering};


use spellcheckprovider::{ISpellCheckProviderFactory, ISpellCheckProviderFactoryVtbl, ISpellCheckProvider, ISpellCheckProviderVtbl};

use com_impl::{ComInterface, interface, implementation};

use std::ffi::OsString;
use ::util::u16_ptr_to_string;

use super::EnumString::EnumString;

#[interface(ISpellCheckProviderFactory)]
pub struct DivvunSpellCheckProviderFactory {
    refs: AtomicU32,
}

#[implementation(IUnknown)]
impl DivvunSpellCheckProviderFactory {
    fn QueryInterface(&mut self, riid: &GUID, obj: &mut usize) -> HRESULT {
        use winapi::shared::winerror::{E_NOTIMPL, S_OK};
        use winapi::Interface;

        *obj = 0;

        if IsEqualGUID(riid, &ISpellCheckProviderFactory::uuidof()) || IsEqualGUID(riid, &IUnknown::uuidof()) {
            *obj = self as *mut _ as usize;
            self.AddRef();
            S_OK
        } else {
            E_NOTIMPL
        }
    }

    fn AddRef(&mut self) -> ULONG {
        let prev = self.refs.fetch_add(1, Ordering::SeqCst);
        prev + 1
    }

    fn Release(&mut self) -> ULONG {
        let prev = self.refs.fetch_sub(1, Ordering::SeqCst);
        if prev == 1 {
            let _box = unsafe { Box::from_raw(self as *mut _) };
        }
        prev - 1
    }
}

#[implementation(ISpellCheckProviderFactory)]
impl DivvunSpellCheckProviderFactory {
    fn get_SupportedLanguages(&mut self, value: *mut *mut IEnumString) -> HRESULT {
        info!("get supported languages");
        let enum_if = EnumString::new(vec!["en".to_string(), "en-us".to_string(), "sv".to_string(), "sv-SE".to_string()]);
        unsafe {
            *value = enum_if as *mut _;
        }
        S_OK
    }

    fn IsSupported(&mut self, LanguageTag: LPCWSTR, value: *mut i32) -> HRESULT {
        unsafe {
            let tag = u16_ptr_to_string(LanguageTag);
            info!("is supported {:?}", tag);
            *value = FALSE;
        }
        S_OK
    }

    fn CreateSpellCheckProvider(&mut self, LanguageTag: LPCWSTR, value: *mut *mut ISpellCheckProvider) -> HRESULT {
        unsafe {
            let tag = u16_ptr_to_string(LanguageTag);
            info!("create spell check provider {:?}", tag);
        }
        //S_OK
        E_INVALIDARG
    }
}


impl DivvunSpellCheckProviderFactory {
    pub fn new() -> *mut DivvunSpellCheckProviderFactory {
        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1)
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}