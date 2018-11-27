use glob::{glob_with, MatchOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use std::sync::{Arc, RwLock};
use hfstospell::archive::SpellerArchive;

use util;

pub struct SpellerRepository {
    base_directory: String,
}

impl SpellerRepository {
    pub fn new(base_directory: &str) -> Self {
        SpellerRepository {
            base_directory: base_directory.to_string()
        }
    }

    pub fn get_speller_archives(&self) -> Vec<PathBuf> {
        let mut path: PathBuf = [&self.base_directory, "**/*.zhfst"].iter().collect();
        info!("Enumerate dictionaries in {:?}", path.display());
        glob_with(path.to_str().unwrap(), &MatchOptions {
            case_sensitive: false,
            require_literal_leading_dot: false,
            require_literal_separator: false
        }).map(|paths|
            paths.filter_map(|i| i.ok())
        ).unwrap().collect()
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        info!("Resolve supported languages");
        self.get_speller_archives().iter().filter_map(|path| {
            path.file_stem().and_then(|path| util::resolve_locale_name(&path.to_string_lossy()))
        }).collect()
    }

    pub fn get_speller_archive(&self, language_tag: &str) -> Option<PathBuf> {
        info!("Resolve supported languages");
        for path in self.get_speller_archives() {
            let tag_name = path.file_stem().and_then(|path| util::resolve_locale_name(&path.to_string_lossy()));
            match tag_name {
                Some(tag_name) => return Some(path),
                _ => ()
            }
        }

        None
    }

    // pub fn get_speller(&mut self, language_tag: &str) -> Option<Arc<SpellerArchive<'data>>> {
    //     let path = self.dictionaries.get(language_tag)?;

    //     Some(self.speller_cache.entry(path.to_string()).or_insert_with(|| {
    //         Arc::new(SpellerArchive::new(path).unwrap())
    //     }).clone())
    // }
}