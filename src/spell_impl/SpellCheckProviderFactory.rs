#![cfg(windows)] 
#![allow(non_snake_case)]
#![allow(unused_variables)]

use winapi::shared::guiddef::{IsEqualGUID, GUID};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{E_INVALIDARG, E_POINTER, S_OK};
use winapi::um::objidlbase::IEnumString;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::{HRESULT, LPCWSTR};

use std::sync::atomic::{AtomicU32, Ordering};

use spellcheckprovider::{
    ISpellCheckProvider, ISpellCheckProviderFactory, ISpellCheckProviderFactoryVtbl,
};

use com_impl::{implementation, interface, ComInterface};

use super::EnumString::EnumString;
use super::SpellCheckProvider::DivvunSpellCheckProvider;

use SPELLER_REPOSITORY;

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

        unsafe {
            *value = supported as i32;
        }

        S_OK
    }

    fn CreateSpellCheckProvider(
        &mut self,
        LanguageTag: LPCWSTR,
        value: *mut *mut ISpellCheckProvider,
    ) -> HRESULT {
        let tag = com_wstr_ptr!(LanguageTag);
        let langs = SPELLER_REPOSITORY.get_supported_languages();
        let supported = langs.contains(&tag);

        if !supported {
            return E_INVALIDARG;
        }

        info!("create spell check provider {:?}", tag);

        if let Some(provider) = DivvunSpellCheckProvider::new(&tag) {
            unsafe {
                *value = provider as *mut _;
            }
            S_OK
        } else {
            info!("spell check provider creation failed");
            // Spell check provider creation failed for some reason we can't communicate anyway
            E_INVALIDARG
        }
    }
}

impl DivvunSpellCheckProviderFactory {
    pub fn new() -> *mut DivvunSpellCheckProviderFactory {
        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}
