#![cfg(windows)]
#![allow(non_snake_case)]

use winapi::ctypes::c_void;
use winapi::shared::guiddef::{IsEqualGUID, GUID};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{E_INVALIDARG, E_POINTER, S_FALSE, S_OK};
use winapi::shared::wtypesbase::LPOLESTR;
use winapi::um::combaseapi::CoTaskMemFree;
use winapi::um::objidlbase::IEnumString;
use winapi::um::winnt::{HRESULT, LPCWSTR};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::spellcheckprovider::{IEnumSpellingError, ISpellCheckProvider, ISpellCheckProviderVtbl};

use com_impl::{implementation, interface, ComInterface};

use divvunspell::archive::{self, SpellerArchive};

use std::collections::HashMap;

use crate::speller_cache::SpellerCache;
use crate::util;
use crate::wordlists::Wordlists;
use crate::SPELLER_REPOSITORY;

use super::EnumSpellingError::DivvunEnumSpellingError;
use super::EnumString::EnumString;

ENUM! {enum WORDLIST_TYPE {
  WORDLIST_TYPE_IGNORE = 0,
  WORDLIST_TYPE_ADD = 1,
  WORDLIST_TYPE_EXCLUDE = 2,
  WORDLIST_TYPE_AUTOCORRECT = 3,
}}

#[interface(ISpellCheckProvider)]
pub struct DivvunSpellCheckProvider {
    refs: AtomicU32,
    language_tag: String,
    speller_archive: Arc<dyn SpellerArchive + Send + Sync>,
    speller_cache: Arc<SpellerCache>,
    wordlists: Arc<Wordlists>,
}

IMPL_UNKNOWN!(ISpellCheckProvider, DivvunSpellCheckProvider);

#[implementation(ISpellCheckProvider)]
impl DivvunSpellCheckProvider {
    fn get_LanguageTag(&mut self, value: *mut LPCWSTR) -> HRESULT {
        info!("get_LanguageTag");
        unsafe {
            *value = util::alloc_com_str(self.language_tag.clone()).unwrap();
        }
        S_OK
    }

    fn Check(&mut self, text: LPCWSTR, value: *mut *mut IEnumSpellingError) -> HRESULT {
        let text = com_wstr_ptr!(text);

        info!("Check {}", text);

        let enum_err = DivvunEnumSpellingError::new(
            self.speller_cache.to_owned(),
            self.wordlists.to_owned(),
            text,
        );
        unsafe {
            *value = enum_err as *mut _;
        }

        S_OK
    }

    fn Suggest(&mut self, word: LPCWSTR, value: *mut *mut IEnumString) -> HRESULT {
        let word = com_wstr_ptr!(word);

        info!("Suggest {}", word);

        let mut suggestions: Vec<String> = vec![];
        let mut result: Option<HRESULT> = None;

        // Check ignore wordlist
        if self.wordlists.contains_ignore(&word) {
            info!("wordlist ignore");
            suggestions.push(word.clone());
            result = Some(S_FALSE);
        }

        // Check auto correct wordlist. TODO: should the speller's suggestions be appended to this perhaps?
        if result.is_none() {
            if let Some(replacement) = self.wordlists.get_replacement(&word) {
                info!("wordlist replacement {}", replacement);
                suggestions.push(replacement);
                result = Some(S_OK);
            }
        }

        // Check add wordlist
        if result.is_none() && self.wordlists.contains_add(&word) {
            info!("wordlist add");
            suggestions.push(word.clone());
            result = Some(S_OK);
        }

        // Check speller result
        if result.is_none() {
            result = Some(S_OK);
            if !self.speller_cache.is_correct(&word) {
                let speller_suggestions = self.speller_cache.to_owned().suggest_cache_only(&word);
                if let Some(speller_suggestions) = speller_suggestions {
                    suggestions = speller_suggestions;
                    info!(
                        "speller {} suggestions: {:?}",
                        suggestions.len(),
                        suggestions
                    );
                } else {
                    // No results available yet but word is incorrect
                    info!("no speller suggestions yet");
                    result = Some(S_OK);
                }
            }

            // No results, word is correct if no excludes in wordlist
            if suggestions.is_empty() {
                // Check exclude wordlist (no suggestions but the word is incorrect)
                if !self.wordlists.contains_exclude(&word) {
                    info!("wordlist exclude");
                    suggestions.push(word.clone());
                    result = Some(S_FALSE);
                }
            }
        }

        let enum_if = EnumString::new(suggestions);
        unsafe {
            *value = enum_if as *mut _;
        }

        result.unwrap()
    }

    fn GetOptionValue(&mut self, optionId: LPCWSTR, value: *mut u8) -> HRESULT {
        info!("GetOptionValue");
        // nope
        E_INVALIDARG
    }

    fn SetOptionValue(&mut self, optionId: LPCWSTR, value: u8) -> HRESULT {
        info!("SetOptionValue");
        // nope
        E_INVALIDARG
    }

    fn get_OptionIds(&mut self, value: *mut *mut IEnumString) -> HRESULT {
        info!("get_OptionIds");
        let enum_if = EnumString::new(vec![]);
        unsafe {
            *value = enum_if as *mut _;
        }
        S_OK
    }

    fn get_Id(&mut self, value: *mut LPCWSTR) -> HRESULT {
        info!("get_Id");
        // divvun or so
        unsafe {
            *value = util::alloc_com_str("windivvun").unwrap();
        }
        S_OK
    }

    fn get_LocalizedName(&mut self, value: *mut LPCWSTR) -> HRESULT {
        info!("get_LocalizedName");
        // Divvun Spell Thing
        unsafe {
            *value = util::alloc_com_str("WinDivvun").unwrap();
        }
        S_OK
    }

    fn GetOptionDescription(&mut self, optionId: LPCWSTR, value: *mut *mut c_void) -> HRESULT {
        info!("GetOptionDescription");
        // nope
        E_INVALIDARG
    }

    fn InitializeWordlist(
        &mut self,
        wordlistType: WORDLIST_TYPE,
        words: *const IEnumString,
    ) -> HRESULT {
        info!("InitializeWordlist {}", wordlistType);
        let elem_count: u32 = 50;
        let mut words_vec: Vec<String> = vec![];
        let mut fetched: ULONG = 0;
        let mut vec: Vec<LPOLESTR> = vec![std::ptr::null_mut(); elem_count as usize];
        loop {
            let res = unsafe { (*words).Next(elem_count, vec.as_mut_ptr(), &mut fetched) };
            if res == S_OK || res == S_FALSE {
                for i in 0..fetched {
                    let word = unsafe { util::u16_ptr_to_string(vec[i as usize]) };
                    unsafe {
                        CoTaskMemFree(vec[i as usize] as *mut c_void);
                    }
                    words_vec.push(word.into_string().unwrap());
                }
            }

            if res != S_OK {
                break;
            }
        }

        match wordlistType {
            WORDLIST_TYPE_ADD => self.wordlists.set_add(words_vec),
            WORDLIST_TYPE_EXCLUDE => self.wordlists.set_exclude(words_vec),
            WORDLIST_TYPE_AUTOCORRECT => {
                let mut map: HashMap<String, String> = HashMap::new();
                for word in words_vec {
                    let tokens = word.split('\t').collect::<Vec<&str>>();
                    if tokens.len() == 2 {
                        map.insert(tokens[0].to_string(), tokens[1].to_string());
                    } else {
                        error!("Invalid auto correct pair: {:?}", tokens);
                    }
                }
                self.wordlists.set_auto_correct(map)
            }
            WORDLIST_TYPE_IGNORE => self.wordlists.set_ignore(words_vec),
            _ => info!("invalid wordlist type {}", wordlistType),
        };

        S_OK
    }
}

impl DivvunSpellCheckProvider {
    pub fn new(language_tag: &str) -> Option<*mut DivvunSpellCheckProvider> {
        let archive_path = SPELLER_REPOSITORY.get_speller_archive(language_tag)?;
        // let archive_path = archive_path.to_str()?;
        // let archive_path = archive_path.to_string();

        info!(
            "Instantiating speller {} at {}",
            language_tag,
            archive_path.display()
        );

        let archive = match archive::open(&archive_path) {
            Ok(v) => v,
            Err(err) => {
                error!("failed to load speller archive: {:?}", err);
                return None;
            }
        };

        let s = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            language_tag: language_tag.to_string(),
            speller_cache: SpellerCache::new(archive.speller()),
            speller_archive: archive,
            wordlists: Wordlists::new(),
        };

        let ptr = Box::into_raw(Box::new(s));

        Some(ptr as *mut _)
    }
}
