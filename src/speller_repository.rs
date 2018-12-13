use std::{io, fs};
use std::path::{Path, PathBuf};

use crate::util;

pub struct SpellerRepository {
    base_directories: Vec<String>,
}

fn find_zhfsts(dir: &Path) -> Vec<PathBuf> {
    let mut results: Vec<PathBuf> = vec!();

    fn visit_dirs(dir: &Path, results: &mut Vec<PathBuf>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, results)?;
                } else {
                    if let Some(ext) = path.extension() {
                        if ext == "zhfst" {
                           results.push(path.to_owned());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    
    let err = visit_dirs(dir, &mut results);
    if let Err(e) = err {
        error!("Error listing {:?}: {:?}", dir, e);
    }

    results
}

impl SpellerRepository {
    pub fn new(base_directories: Vec<String>) -> Self {
        SpellerRepository { base_directories }
    }

    pub fn get_speller_archives(&self) -> Vec<PathBuf> {
        self.base_directories
            .iter()
            .flat_map(|base_directory| {
                // let path: PathBuf = [base_directory, "**/*.zhfst"].iter().collect();
                info!("Enumerate dictionaries in {:?}", base_directory);

                // glob_with(
                //     path.to_str().unwrap(),
                //     &MatchOptions {
                //         case_sensitive: false,
                //         require_literal_leading_dot: false,
                //         require_literal_separator: false,
                //     },
                // )
                // .map(|paths|
                //     paths.inspect(|p| info!("path: {:?}", p)).filter_map(|i| i.ok()))
                // .unwrap()
                find_zhfsts(&Path::new(base_directory))
            })
            .collect()
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        info!("Resolve supported languages");
        self.get_speller_archives()
            .iter()
            .filter_map(|path| {
                path.file_stem()
                    .and_then(|path| util::resolve_locale_name(&path.to_string_lossy()))
            })
            .collect()
    }

    pub fn get_speller_archive(&self, language_tag: &str) -> Option<PathBuf> {
        info!("Resolve supported languages");
        for path in self.get_speller_archives() {
            let tag_name = path
                .file_stem()
                .and_then(|path| util::resolve_locale_name(&path.to_string_lossy()));
            if let Some(tag_name) = tag_name {
                if tag_name == language_tag {
                    return Some(path);
                }
            }
        }

        None
    }
}
