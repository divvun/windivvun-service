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

IMPL_UNKNOWN!(ISpellCheckProviderFactory, DivvunSpellCheckProviderFactory);

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
        let tag = com_wstr_ptr!(LanguageTag);

        let langs = SPELLER_REPOSITORY.get_supported_languages();
        let supported = langs.contains(&tag);

        info!("is supported {:?}: {}", tag, supported);
        
        unsafe { *value = supported as i32; }

        S_OK
    }

    fn CreateSpellCheckProvider(&mut self, LanguageTag: LPCWSTR, value: *mut *mut ISpellCheckProvider) -> HRESULT {
        let tag = com_wstr_ptr!(LanguageTag);
        let langs = SPELLER_REPOSITORY.get_supported_languages();
        let supported = langs.contains(&tag);
        
        if !supported {
            return E_INVALIDARG
        }

        info!("create spell check provider {:?}", tag);

        let provider = DivvunSpellCheckProvider::new(&tag);

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