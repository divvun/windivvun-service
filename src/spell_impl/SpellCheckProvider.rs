#![cfg(windows)] 
#![allow(non_snake_case)]

use winapi::um::objidlbase::IEnumString;
use winapi::um::winnt::{LPCWSTR, HRESULT};
use winapi::shared::ntdef::ULONG;
use winapi::shared::winerror::{S_OK, E_INVALIDARG, E_POINTER, S_FALSE};
use winapi::shared::guiddef::{IsEqualGUID, GUID};
use winapi::ctypes::c_void;
use winapi::shared::wtypesbase::{LPOLESTR, OLECHAR};
use winapi::um::combaseapi::CoTaskMemFree;

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use spellcheckprovider::{ISpellCheckProvider, ISpellCheckProviderVtbl, IEnumSpellingError};

use com_impl::{ComInterface, interface, implementation};

use hfstospell::speller::{Speller, SpellerConfig};
use hfstospell::archive::SpellerArchive;

use ::SPELLER_REPOSITORY;
use ::util;
use ::speller_cache::SpellerCache;

use super::EnumString::EnumString;
use super::EnumSpellingError::DivvunEnumSpellingError;

ENUM!{enum WORDLIST_TYPE {
  WORDLIST_TYPE_IGNORE = 0,
  WORDLIST_TYPE_ADD = 1,
  WORDLIST_TYPE_EXCLUDE = 2,
  WORDLIST_TYPE_AUTOCORRECT = 3,
}}

#[interface(ISpellCheckProvider)]
pub struct DivvunSpellCheckProvider {
    refs: AtomicU32,
    language_tag: String,
    speller: Arc<Speller>,
    speller_cache: Arc<SpellerCache>
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

    let enum_err = DivvunEnumSpellingError::new(self.speller_cache.to_owned(), text);
    unsafe { *value = enum_err as *mut _; }

    S_OK
  }

  fn Suggest(&mut self, word: LPCWSTR, value: *mut *mut IEnumString) -> HRESULT {
    let word = com_wstr_ptr!(word);

    info!("Suggest {}", word);

    let mut suggestions = self.speller_cache.to_owned().suggest(&word);
    info!("{} suggestions: {:?}", suggestions.len(), suggestions);

    //std::thread::sleep(std::time::Duration::from_millis(2000));

    let mut result: HRESULT = S_OK;
    if suggestions.len() == 0 {
      suggestions.push(word);
      result = S_FALSE;
    }

    let enum_if = EnumString::new(suggestions);
    unsafe { *value = enum_if as *mut _; }
    
    result
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
    // return empty list
    // or: SpellerConfig
    // pub n_best: Option<usize>,
    // pub max_weight: Option<Weight>,
    // pub beam: Option<Weight>,
    let enum_if = EnumString::new(vec![]);
    unsafe { *value = enum_if as *mut _; }
    S_OK
  }

  fn get_Id(&mut self, value: *mut LPCWSTR) -> HRESULT {
    info!("get_Id");
    // divvun or so
    unsafe {
      *value = util::alloc_com_str("divvun").unwrap();
    }
    S_OK
  }

  fn get_LocalizedName(&mut self, value: *mut LPCWSTR) -> HRESULT {
    info!("get_LocalizedName");
    // Divvun Spell Thing
    unsafe {
      *value = util::alloc_com_str("Divvun Spell Checker").unwrap();
    }
    S_OK
  }

  fn GetOptionDescription(&mut self, optionId: LPCWSTR, value: *mut *mut c_void) -> HRESULT {
    info!("GetOptionDescription");
    // nope
    E_INVALIDARG
  }

  fn InitializeWordlist(&mut self, wordlistType: WORDLIST_TYPE, words: *const IEnumString) -> HRESULT {
    info!("InitializeWordlist");
    let elem_count: u32 = 50;
    let mut fetched: ULONG = 0;
    let mut vec: Vec<LPOLESTR> = vec![std::ptr::null_mut(); elem_count as usize];
    loop {
      let res = unsafe { (*words).Next(elem_count, vec.as_mut_ptr(), &mut fetched) };
      if res == S_OK || res == S_FALSE {
        info!("fetched {} elems", fetched);

        for i in (0..fetched) {
          let word = unsafe { util::u16_ptr_to_string(vec[i as usize]) };
          unsafe { CoTaskMemFree(vec[i as usize] as *mut c_void); }

          info!("{}: {:?}", i, word);
        }
      }
      
      if res != S_OK {
        break
      }
    }
    // nope
    // or: keep list of words, check for equalness before invoking speller
    S_OK
  }
}

impl DivvunSpellCheckProvider {
  pub fn new(language_tag: &str) -> *mut DivvunSpellCheckProvider {
    //, archivePath: &str
    let archive_path = SPELLER_REPOSITORY.get_speller_archive(language_tag);
    // TODO
    let archive_path_w = archive_path.unwrap().to_str().unwrap().to_owned();

    info!("Instanciating speller {} at {}", language_tag, archive_path_w);

    let archive = SpellerArchive::new(&archive_path_w).unwrap();
    let speller = archive.speller();
    
    let s = Self {
        __vtable: Box::new(Self::create_vtable()),
        refs: AtomicU32::new(1),
        language_tag: language_tag.to_string(),
        speller: speller.clone(),
        speller_cache: SpellerCache::new(speller)
    };

    let ptr = Box::into_raw(Box::new(s));

    ptr as *mut _
  }
}