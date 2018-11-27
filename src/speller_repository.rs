use std::collections::HashMap;
use std::path::{Path, PathBuf};

use std::sync::{Arc, RwLock};
use hfstospell::archive::SpellerArchive;

struct SpellerRepository<'data> {
    supported_languages: Vec<String>,
    dictionaries: HashMap<String, String>,
    speller_cache: HashMap<String, Arc<SpellerArchive<'data>>>
}

impl<'data> SpellerRepository<'data> {
    pub fn new() -> Self {
        SpellerRepository {
            dictionaries: HashMap::new(),
            supported_languages: Vec::new(),
            speller_cache: HashMap::new()
        }
    }

    pub fn add_dictionary(&mut self, language_tag: &str, dictionary_archive: &str) {
        let language_tag = language_tag.to_string();
        self.supported_languages.push(language_tag);
        self.dictionaries.insert(language_tag, dictionary_archive.to_string());
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        self.supported_languages
    }

    pub fn get_speller(&mut self, language_tag: &str) -> Option<Arc<SpellerArchive<'data>>> {
        let path = self.dictionaries.get(language_tag)?;

        Some(self.speller_cache.entry(path.to_string()).or_insert_with(|| {
            Arc::new(SpellerArchive::new(path).unwrap())
        }).clone())
    }
}