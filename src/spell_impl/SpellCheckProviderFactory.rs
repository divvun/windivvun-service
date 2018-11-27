#![cfg(windows)] 
#![allow(non_snake_case)]
#![allow(unused_variables)]

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
use super::SpellCheckProvider::DivvunSpellCheckProvider;

use ::SPELLER_REPOSITORY;

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
        let langs = SPELLER_REPOSITORY.get_supported_languages();
        let enum_if = EnumString::new(langs);
        unsafe {
            *value = enum_if as *mut _;
        }
        S_OK
    }

    fn IsSupported(&mut self, LanguageTag: LPCWSTR, value: *mut i32) -> HRESULT {
        let tag = unsafe { u16_ptr_to_string(LanguageTag) };
        let langs = SPELLER_REPOSITORY.get_supported_languages();
        let supported = tag.clone().into_string().map(|s| langs.contains(&s)).unwrap_or(false);

        info!("is supported {:?}: {}", tag, supported);
        
        unsafe { *value = supported as i32; }

        S_OK
    }

    fn CreateSpellCheckProvider(&mut self, LanguageTag: LPCWSTR, value: *mut *mut ISpellCheckProvider) -> HRESULT {
        let tag = unsafe { u16_ptr_to_string(LanguageTag) };
        let langs = SPELLER_REPOSITORY.get_supported_languages();
        let tag_str = tag.into_string();
        let supported = tag_str.clone().map(|s| langs.contains(&s)).unwrap_or(false);
        
        if !supported {
            return E_INVALIDARG
        }

        info!("create spell check provider {:?}", tag_str);

        let provider = DivvunSpellCheckProvider::new(&tag_str.unwrap());

        unsafe {
            *value = provider as *mut  _
        }

        S_OK
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