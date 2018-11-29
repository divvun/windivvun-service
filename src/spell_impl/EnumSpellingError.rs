#![cfg(windows)] 
#![allow(non_snake_case)]

use winapi::um::objidlbase::IEnumString;
use winapi::um::winnt::{LPCWSTR, HRESULT};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_INVALIDARG, E_POINTER, S_FALSE};
use winapi::shared::guiddef::{IsEqualGUID, GUID};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::sync::atomic::{AtomicU32, Ordering};


use spellcheckprovider::{IEnumSpellingError, IEnumSpellingErrorVtbl, ISpellingError, ISpellingErrorVtbl, CORRECTIVE_ACTION_REPLACE, CORRECTIVE_ACTION_GET_SUGGESTIONS};

use com_impl::{ComInterface, interface, implementation};

use hfstospell::speller::{SpellerConfig, Speller};
use hfstospell::tokenizer::{Tokenize, Token};

use std::sync::Arc;
use std::ffi::OsString;
use ::util;
use ::speller_cache::SpellerCache;
use ::wordlists::Wordlists;

#[interface(ISpellingError)]
pub struct DivvunSpellingError {
    refs: AtomicU32,
    start_index: u32,
    length: u32,
    corrective_action: u32,
    replacement: Vec<u16>
}

IMPL_UNKNOWN!(ISpellingError, DivvunSpellingError);

#[implementation(ISpellingError)]
impl DivvunSpellingError {
    fn get_StartIndex(&mut self, value: *mut u32) -> HRESULT {
        info!("StartIndex {}", self.start_index);
        unsafe { *value = self.start_index; }
        S_OK
    }

    fn get_Length(&mut self, value: *mut u32) -> HRESULT {
        info!("Length {}", self.length);
        unsafe { *value = self.length; }
        S_OK
    }

    fn get_CorrectiveAction(&mut self, value: *mut u32) -> HRESULT {
        info!("CorrectiveAction {}", self.corrective_action);
        unsafe { *value = self.corrective_action; }
        S_OK
    }

    fn get_Replacement(&mut self, value: *mut LPCWSTR) -> HRESULT {
        info!("Replacement {:?}", self.replacement);
        if self.corrective_action != CORRECTIVE_ACTION_REPLACE {
            unsafe { *value = std::ptr::null_mut(); }
        } else {
            unsafe { *value = self.replacement.as_ptr(); }
        }
        S_OK
    }
}

impl DivvunSpellingError {
    pub fn new(
        start_index: u32,
        length: u32,
        corrective_action: u32,
        replacement: Option<String>
    ) -> *mut DivvunSpellingError {
        info!("create");
        let replacement = replacement.map_or(vec!(), |r| util::to_u16s(r).unwrap_or(vec!()));
        info!("err repl {:?}", replacement);

        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            
            start_index,
            length,
            corrective_action,
            
            replacement
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}

#[interface(IEnumSpellingError)]
pub struct DivvunEnumSpellingError {
    refs: AtomicU32,
    speller_cache: Arc<SpellerCache>,
    text: Arc<String>,
    text_offset: usize,
    wordlists: Arc<Wordlists>
}

IMPL_UNKNOWN!(IEnumSpellingError, DivvunEnumSpellingError);

#[implementation(IEnumSpellingError)]
impl DivvunEnumSpellingError {
    fn Next(&mut self, value: *mut *mut ISpellingError) -> HRESULT {
        info!("Next");

        let tokenizer_start = self.text_offset;
        let tokenizer = self.text[tokenizer_start..].tokenize();
        let tokens = tokenizer.filter_map(|t| match t {
            Token::Word(_, _, _) => Some(t),
            _ => None
        });
        
        for token in tokens {
            info!("Token {:?}", token);
            self.text_offset = tokenizer_start + token.end();

            // Check ignore wordlist
            if self.wordlists.contains_ignore(token.value()) {
                info!("Wordlist ignore");
                continue;
            }

            let mut action: Option<u32> = None;
            let mut replacement: Option<String> = None;

            // Check auto correct wordlist
            match self.wordlists.get_replacement(token.value()) {
                Some(r) => {
                    info!("wordlist replace {}", r);

                    action = Some(CORRECTIVE_ACTION_REPLACE);
                    replacement = Some(r);  
                },
                _ => ()
            };

            // Check exclude wordlist
            if action.is_none() && self.wordlists.contains_exclude(token.value()) {
                info!("wordlist incorrect");
                action = Some(CORRECTIVE_ACTION_GET_SUGGESTIONS);
                replacement = None;
            }

            // Check add wordlist
            if self.wordlists.contains_add(token.value()) {
                info!("wordlist added");
                action = None;
            } else {
                // Query speller API
                if action.is_none() && !self.speller_cache.to_owned().is_correct(token.value()) {
                    action = Some(CORRECTIVE_ACTION_GET_SUGGESTIONS);
                    replacement = None;
                }
            }

            if action.is_none() {
                continue;
            }

            self.speller_cache.prime(token.value());
            let error = DivvunSpellingError::new(
                (tokenizer_start + token.start()) as u32,
                (token.end() - token.start()) as u32,
                action.unwrap(),
                replacement
            );

            info!("error {} {}", (tokenizer_start + token.start()) as u32, (token.end() - token.start()) as u32);

            unsafe { *value = error as *mut _; }
            info!("return");
            return S_OK;
        }

        S_FALSE
    }
}

impl DivvunEnumSpellingError {
    pub fn new(speller_cache: Arc<SpellerCache>, wordlists: Arc<Wordlists>, text: String) -> *mut DivvunEnumSpellingError {
        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            speller_cache,
            text: Arc::new(text),
            text_offset: 0,
            wordlists
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}

#[test]
fn tokens() {
    let res: Vec<Token> = "Hello world how are you doing".tokenize().filter_map(|t| match t {
        Token::Word(_, _, _) => Some(t),
        _ => None
    }).collect();

    for r in res {
        println!("{:?}", r);
    }
}