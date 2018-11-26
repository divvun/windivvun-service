use std::collections::HashMap;
use std::path::{Path, PathBuf};

use std::sync::{Arc, RwLock}
use hfstospell::archive::SpellerArchive;

struct SpellerRepository {
    base_path: String,

    supported_languages: Vec<String>,
    dictionaries: HashMap<String, PathBuf>,
    speller_cache: HashMap<PathBuf, Arc<SpellerArchive>>
}

impl SpellerRepository {
    pub fn new(base_path: &str) -> Self {
        SpellerRepository {
            base_path: base_path.to_string(),
            dictionaries: HashMap::new(),
            supported_languages: Vec::new(),
            speller_cache: HashMap::new()
        }
    }

    pub fn add_dictionary(&mut self, language_tag: &str, dictionary_archive: &Path) {
        let language_tag = language_tag.to_string();
        self.supported_languages.push(language_tag);
        self.dictionaries.insert(language_tag, dictionary_archive.to_path_buf());
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        self.supported_languages
    }

    pub fn get_speller(&mut self, language_tag: &str) -> Arc<RwLock<Speller>> {
        let path = self.dictionaries.get(language_tag).unwrap();

        &self.speller_cache.entry(&path).or_insert_with(|| {
            SpellerArchive::new(path)
        }).speller
    }
}