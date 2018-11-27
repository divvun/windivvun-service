#![cfg(windows)] 
#![allow(non_snake_case)]

use winapi::um::objidlbase::IEnumString;
use winapi::um::winnt::{LPCWSTR, HRESULT};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_INVALIDARG, E_POINTER};
use winapi::shared::guiddef::{IsEqualGUID, GUID};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::sync::atomic::{AtomicU32, Ordering};


use spellcheckprovider::{IEnumSpellingError, IEnumSpellingErrorVtbl, CORRECTIVE_ACTION};

use com_impl::{ComInterface, interface, implementation};

use hfstospell::Speller;

use std::string::SplitWhitespace;

use std::ffi::OsString;

#[interface(ISpellingError)]
pub struct DivvunSpellingError {
    refs: AtomicU32,
    start_index: u32,
    length: u32,
    corrective_action: CORRECTIVE_ACTION,
    replacement: Vec<u16>
}

#[implementation(IUnknown)]
impl DivvunSpellingError {
    fn QueryInterface(&mut self, riid: &GUID, obj: &mut usize) -> HRESULT {
        use winapi::shared::winerror::{E_NOTIMPL, S_OK};
        use winapi::Interface;

        *obj = 0;

        if IsEqualGUID(riid, &self::uuidof()) || IsEqualGUID(riid, &IUnknown::uuidof()) {
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

#[implementation(IEnumSpellingError)]
impl DivvunSpellingError {
    fn new(
        start_index: u32,
        length: u32,
        corrective_action: CORRECTIVE_ACTION,
        replacement: String
    ) -> *mut DivvunSpellingError {
        let struct = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            
            start_index,
            length,
            corrective_action,
            
            replacement: util::to_u16s(replacement).expect("a valid replacement string")
        }

        let ptr = Box::into_raw(Box::new(struct));

        ptr as *mut _
    }

    fn get_StartIndex(&mut self, value: *mut u32) -> HRESULT {
        *value = self.start_index
        S_OK
    }

    fn get_Length(&mut self, value: *mut u32) -> HRESULT {
        *value = self.length
        S_OK
    }

    fn get_CorrectiveAction(&mut self, value: *mut CORRECTIVE_ACTION) -> HRESULT {
        *value = self.corrective_action
        S_OK
    }

    fn get_Replacement(&mut self, value: *mut LPCWSTR) -> HRESULT {
        *value = self.replacement.as_ptr();
        S_OK
    }
}

#[interface(IEnumSpellingError)]
pub struct DivvunEnumSpellingError {
    refs: AtomicU32,
    speller: Speller,
    text: String,
    current_offset: u32
}

#[implementation(IUnknown)]
impl DivvunEnumSpellingError {
    fn QueryInterface(&mut self, riid: &GUID, obj: &mut usize) -> HRESULT {
        use winapi::shared::winerror::{E_NOTIMPL, S_OK};
        use winapi::Interface;

        *obj = 0;

        if IsEqualGUID(riid, &self::uuidof()) || IsEqualGUID(riid, &IUnknown::uuidof()) {
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

#[implementation(IEnumSpellingError)]
impl DivvunEnumSpellingError {

    fn Next(&mut self, value: *mut *mut ISpellingError) -> HRESULT {
        // find first delimiter character
        // make an iterator like split_whitespace but with the start index available

        // CORRECTIVE_ACTION_GET_SUGGESTIONS if there's more than one suggestion
        // CORRECTIVE_ACTION_REPLACE if there is one

        // get next spelling error vOv
        let error = DivvunSpellingError {
            speller, 
            current_offset,
            corrective_action: CORRECTIVE_ACTION_GET_SUGGESTIONS
        }

        // S_OK if there is one
        S_FALSE
    }
}

impl DivvunEnumSpellingError {
    fn new(engine: Speller, text: &str) -> *mut DivvunEnumSpellingError {
        let struct = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            speller,
            text,
            current_offset: 0
        }

        let iterator = text.char_indices()
            .filter(|(_, c)| char::is_whitespace(*c))
            .map(|(i, _)| i)
            .chain(std::iter::once(s.len()))
            .scan(0, |start, end| {
                let text = &s[*start..end];
                let length = end - *start;
                let begin = *start;
                *start = end + 1;

                Some(Word {
                    begin, end, length, text
                })
            });

        .filter(|word| !word.text.is_empty());

        let ptr = Box::into_raw(Box::new(struct));

        ptr as *mut _
    }
}