#![cfg(windows)] 

use winapi::um::objidlbase::IEnumString;
use winapi::um::winnt::{LPCWSTR, HRESULT};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_INVALIDARG, E_POINTER};
use winapi::shared::guiddef::{IsEqualGUID, GUID};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::sync::atomic::{AtomicU32, Ordering};


use spellcheckprovider::{ISpellCheckProviderFactory, ISpellCheckProviderFactoryVtbl, ISpellCheckProvider, ISpellCheckProviderVtbl};

use com_impl::{ComInterface, interface, implementation};


#[interface(ISpellCheckProviderFactory)]
pub struct DivvunSpellCheckProviderFactory {
    refs: AtomicU32,
    variable: u64,
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
        S_OK
    }

    fn IsSupported(&mut self, LanguageTag: LPCWSTR, value: *mut i32) -> HRESULT {
        S_OK
    }

    fn CreateSpellCheckProvider(&mut self, LanguageTag: LPCWSTR, value: *mut *mut ISpellCheckProvider) -> HRESULT {
        S_OK
    }
}