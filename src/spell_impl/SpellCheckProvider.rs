#![cfg(windows)] 
#![feature(integer_atomics)]

use winapi::um::objidlbase::IEnumString;
use winapi::um::winnt::{LPCWSTR, HRESULT};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_INVALIDARG, E_POINTER};
use winapi::shared::guiddef::{IsEqualGUID, GUID};

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::sync::atomic::{AtomicU32, Ordering};


use spellcheckprovider::{ISpellCheckProvider, ISpellCheckProviderVtbl};

use com_impl::{ComInterface, interface, implementation};

use hfstospell::Speller;

#[interface(ISpellCheckProvider)]
pub struct DivvunSpellCheckProvider {
    refs: AtomicU32,
    languageTag: String,
    engine: Speller
}

#[implementation(IUnknown)]
impl DivvunSpellCheckProvider {
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

#[implementation(ISpellCheckProvider)]
impl DivvunSpellCheckProvider {
  fn get_LanguageTag(&mut self, value: *mut LPCWSTR) -> HRESULT {
    value = languageTag;
  }

  fn Check(&mut self, text: LPCWSTR, value: *mut *mut IEnumSpellingError) -> HRESULT {
    // run hf on entire text ???
    // split by word?
    // delimeters: ' ', '\t', '\n'
  }

  fn Suggest(&mut self, word: LPCWSTR, value: *mut *mut IEnumString) -> HRESULT {
    // self.speller.is_correct(text)
    let suggestions = self.speller.suggest(word);
    // sort suggestions
    // make IEnumString
  }

  fn GetOptionValue(&mut self, optionId: LPCWSTR, value: *mut u8) -> HRESULT {
    // nope
  }

  fn SetOptionValue(&mut self, optionId: LPCWSTR, value: u8) -> HRESULT {
    // nope
  }

  fn get_OptionIds(&mut self, value: *mut *mut IEnumString) -> HRESULT {
    // return empty list
    // or: SpellerConfig
    // pub n_best: Option<usize>,
    // pub max_weight: Option<Weight>,
    // pub beam: Option<Weight>,
    
  }

  fn get_Id(&mut self, value: *mut LPCWSTR) -> HRESULT {
    // divvun or so
  }

  fn get_LocalizedName(&mut self, value: *mut LPCWSTR) -> HRESULT {
    // Divvun Spell Thing
  }

  fn GetOptionDescription(&mut self, optionId: LPCWSTR, value: *mut *mut IOptionDescription) -> HRESULT {
    // nope
  }

  fn InitializeWordlist(&mut self, wordlistType: WORDLIST_TYPE, words: *const IEnumString) -> HRESULT {
    // nope
    // or: keep list of words, check for equalness before invoking speller
  }
}

impl DivvunSpellCheckProviderFactory {
  pub fn new(languageTag: &str) -> *mut DivvunSpellCheckProviderFactory {
    //, archivePath: &str
    let zhfst = SpellerArchive::new(archivePath);
    let speller = zhfst.speller();
    
    let s = Self {
        __vtable: Box::new(Self::create_vtable()),
        refs: AtomicU32::new(1),
        languageTag,
        speller
    };

    let ptr = Box::into_raw(Box::new(s));

    ptr as *mut _
  }
}