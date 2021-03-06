// use divvunspell::archive::zip::HfstZipSpeller;
use divvunspell::speller::{Speller, SpellerConfig};

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;

pub struct SpellerCache {
    speller: Arc<dyn Speller + Send + Sync>,
    //speller_config: SpellerConfig,
    is_correct: RwLock<HashMap<String, bool>>,
    suggestions: Arc<RwLock<HashMap<String, Vec<String>>>>,
    sender: Sender<String>,
}

fn suggest_internal(speller: &Arc<dyn Speller + Send + Sync>, word: &str) -> Vec<String> {
    let speller_config = SpellerConfig::default();

    let res: Vec<String> = speller
        .to_owned()
        .suggest_with_config(word, &speller_config)
        .iter()
        .map(|s| s.value().to_string())
        .collect();
    res
}

impl SpellerCache {
    pub fn new(speller: Arc<dyn Speller + Send + Sync>) -> Arc<Self> {
        let (tx, rx) = channel();

        let result = Arc::new(Self {
            sender: tx,
            speller: speller.clone(),
            is_correct: RwLock::new(HashMap::new()),
            suggestions: Arc::new(RwLock::new(HashMap::new())),
        });

        {
            let suggestions = result.suggestions.clone();
            thread::spawn(move || loop {
                match rx.recv() {
                    Ok(word) => {
                        info!("Received prime for {}", word);
                        // Prime the word
                        if suggestions.read().contains_key(&word) {
                            continue;
                        }
                        let result = suggest_internal(&speller, &word);

                        let mut lock = suggestions.write();
                        lock.insert(word.to_string(), result);
                        info!("Primed {}", word);
                    }
                    err => {
                        info!("Prime loop ending: {:?}", err);
                        return;
                    }
                }
            });
        }

        result
    }

    pub fn prime(self: &Arc<Self>, word: &str) {
        info!("Attempting to prime {}", word);
        if !self.suggestions.read().contains_key(word)
            && self.sender.send(word.to_string()).is_err()
        {
            error!("Failed to send prime word");
        }
    }

    pub fn is_correct(self: &Arc<Self>, word: &str) -> bool {
        {
            let lock = self.is_correct.read();
            let result = lock.get(word);
            if result.is_some() {
                return *result.unwrap();
            }
        }

        let is_correct = self.speller.to_owned().is_correct(word);
        self.is_correct.write().insert(word.to_string(), is_correct);
        is_correct
    }

    pub fn suggest_cache_only(self: &Arc<Self>, word: &str) -> Option<Vec<String>> {
        let lock = self.suggestions.read();
        let result = lock.get(word);
        if result.is_none() {
            self.prime(word);
        }

        result.cloned()
    }

    pub fn suggest(self: &Arc<Self>, word: &str) -> Vec<String> {
        {
            let lock = self.suggestions.read();
            let result = lock.get(word);
            if result.is_some() {
                return result.unwrap().to_owned();
            }
        }

        let result = suggest_internal(&self.speller, word);
        self.suggestions
            .write()
            .insert(word.to_string(), result.to_owned());
        result
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         use divvunspell::archive::SpellerArchive;
//         let archive =
//             SpellerArchive::new(r"C:\Program Files\SpellCheckTest\dicts\se.zhfst").unwrap();
//         let speller = archive.speller();

//         let cache = SpellerCache::new(speller);
//         println!("cache A {:?}", cache.to_owned().is_correct("hello"));
//         println!("cache A2 {:?}", cache.to_owned().is_correct("hello"));
//         println!("cache B {:?}", cache.to_owned().suggest("hello"));
//         println!("cache B2 {:?}", cache.to_owned().suggest("hello"));
//     }
// }
