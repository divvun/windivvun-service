#![cfg(windows)]
#![allow(non_snake_case)]

use winapi::shared::guiddef::{IsEqualGUID, GUID};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_FALSE, S_OK};
use winapi::um::winnt::{HRESULT, LPCWSTR};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::sync::atomic::{AtomicU32, Ordering};

use crate::spellcheckprovider::{
    IEnumSpellingError, IEnumSpellingErrorVtbl, ISpellingError, ISpellingErrorVtbl,
    CORRECTIVE_ACTION_GET_SUGGESTIONS, CORRECTIVE_ACTION_REPLACE,
};

use com_impl::{implementation, interface, ComInterface};

use divvunspell::tokenizer::Tokenize;

use crate::speller_cache::SpellerCache;
use crate::util;
use crate::wordlists::Wordlists;
use std::sync::Arc;

#[interface(ISpellingError)]
pub struct DivvunSpellingError {
    refs: AtomicU32,
    start_index: u32,
    length: u32,
    corrective_action: u32,
    replacement: Vec<u16>,
}

IMPL_UNKNOWN!(ISpellingError, DivvunSpellingError);

#[implementation(ISpellingError)]
impl DivvunSpellingError {
    fn get_StartIndex(&mut self, value: *mut u32) -> HRESULT {
        // info!("StartIndex, {:?}", self.start_index);
        unsafe {
            *value = self.start_index;
        }
        S_OK
    }

    fn get_Length(&mut self, value: *mut u32) -> HRESULT {
        // info!("Length, {:?}", self.length);
        unsafe {
            *value = self.length;
        }
        S_OK
    }

    fn get_CorrectiveAction(&mut self, value: *mut u32) -> HRESULT {
        // info!("CorrectiveAction, {:?}", self.corrective_action);
        unsafe {
            *value = self.corrective_action;
        }
        S_OK
    }

    fn get_Replacement(&mut self, value: *mut LPCWSTR) -> HRESULT {
        if self.corrective_action != CORRECTIVE_ACTION_REPLACE {
            // info!("Replacement, null");
            unsafe {
                *value = std::ptr::null_mut();
            }
        } else {
            // info!("Replacement, something");
            unsafe {
                *value = self.replacement.as_ptr();
            }
        }
        S_OK
    }
}

impl DivvunSpellingError {
    pub fn new(
        start_index: u32,
        length: u32,
        corrective_action: u32,
        replacement: Option<String>,
    ) -> *mut DivvunSpellingError {
        let replacement =
            replacement.map_or(vec![], |r| util::to_u16s(r).unwrap_or_else(|_| vec![]));

        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),

            start_index,
            length,
            corrective_action,

            replacement,
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}

fn byte_off_to_idx(text: &str, byte_offset: usize) -> usize {
    text.char_indices()
        .enumerate()
        .find(|(i, (n, _))| *n == byte_offset)
        .map(|(i, _)| i)
        .unwrap_or(text.chars().count())
}

#[interface(IEnumSpellingError)]
pub struct DivvunEnumSpellingError {
    refs: AtomicU32,
    speller_cache: Arc<SpellerCache>,
    text: Arc<String>,
    text_offset: usize,
    wordlists: Arc<Wordlists>,
}

IMPL_UNKNOWN!(IEnumSpellingError, DivvunEnumSpellingError);

#[implementation(IEnumSpellingError)]
impl DivvunEnumSpellingError {
    fn Next(&mut self, value: *mut *mut ISpellingError) -> HRESULT {
        info!("Next");

        let tokenizer_start = self.text_offset;

        let tokens = self.text[tokenizer_start..]
            .word_bound_indices()
            .filter(|(i, s)| s.chars().any(|ch| ch.is_alphanumeric()));

        for (start, word) in tokens {
            info!("Token {}: {:?}", start, word);
            self.text_offset = tokenizer_start + start + word.len();

            // Check ignore wordlist
            if self.wordlists.contains_ignore(word) {
                info!("Wordlist ignore");
                continue;
            }

            let mut action: Option<u32> = None;
            let mut replacement: Option<String> = None;

            // Check auto correct wordlist
            if let Some(r) = self.wordlists.get_replacement(word) {
                info!("wordlist replace {}", r);

                action = Some(CORRECTIVE_ACTION_REPLACE);
                replacement = Some(r);
            }

            // Check exclude wordlist
            if action.is_none() && self.wordlists.contains_exclude(word) {
                info!("wordlist incorrect");
                action = Some(CORRECTIVE_ACTION_GET_SUGGESTIONS);
                replacement = None;
            }

            // Check add wordlist
            if self.wordlists.contains_add(word) {
                info!("wordlist added");
                action = None;
            } else {
                // Query speller API
                if action.is_none() && !self.speller_cache.to_owned().is_correct(word) {
                    action = Some(CORRECTIVE_ACTION_GET_SUGGESTIONS);
                    replacement = None;
                }
            }

            if action.is_none() {
                continue;
            }

            info!(
                "word {:?}, error action: {:?}, replacement {:?}",
                word, action, replacement
            );

            self.speller_cache.prime(word);

            info!("Rstart {}, Rlength {}", start, word.len());

            let start = byte_off_to_idx(&self.text, tokenizer_start + start);
            let length = byte_off_to_idx(&self.text, start + word.len())
                - byte_off_to_idx(&self.text, start);

            info!("start {}, length {}", start, length);

            let error = DivvunSpellingError::new(
                start as u32,
                length as u32,
                action.unwrap(),
                replacement.to_owned(),
            );

            unsafe {
                *value = error as *mut _;
            }
            return S_OK;
        }

        S_FALSE
    }
}

impl DivvunEnumSpellingError {
    pub fn new(
        speller_cache: Arc<SpellerCache>,
        wordlists: Arc<Wordlists>,
        text: String,
    ) -> *mut DivvunEnumSpellingError {
        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            speller_cache,
            text: Arc::new(text),
            text_offset: 0,
            wordlists,
        };

        let ptr = Box::into_raw(Box::new(s));

        ptr as *mut _
    }
}

#[test]
fn tokens() {
    let res: Vec<Token> = "Hello world how are you doing"
        .tokenize()
        .filter_map(|t| {
            if let Token::Word(_, _, _) = t {
                Some(t)
            } else {
                None
            }
        })
        .collect();

    for r in res {
        println!("{:?}", r);
    }
}
