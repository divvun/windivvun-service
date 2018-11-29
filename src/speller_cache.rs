use hfstospell::speller::{Speller, SpellerConfig};
use hfstospell::archive::SpellerArchive;

use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

pub struct SpellerCache {
    speller: Arc<Speller>,
    //speller_config: SpellerConfig,
    is_correct: RwLock<HashMap<String, bool>>,
    suggestions: RwLock<HashMap<String, Vec<String>>>,
    sender: Sender<String>
}

impl SpellerCache {
    pub fn new(speller: Arc<Speller>) -> Arc<Self> {
        let (tx, rx) = channel();

        let result = Arc::new(Self {
            sender: tx,
            speller: speller.clone(),
            is_correct: RwLock::new(HashMap::new()),
            suggestions: RwLock::new(HashMap::new()),
        });

        {
            // let cache = result.clone();
            // thread::spawn(move || loop {
            //     match rx.recv() {
            //         Ok(word) => {
            //             info!("Received prime for {}", word);
            //             // Prime the word
            //             let lock = cache.suggestions.write().unwrap();
            //             if lock.contains_key(&word) {
            //                 continue;
            //             }
            //             let result = cache.suggest_internal(&word);
            //             lock.insert(word.to_string(), result);
            //             info!("Primed {}", word);
            //         },
            //         _ => ()
            //     }
            // });
        }

        result
    }

    pub fn prime(self: Arc<Self>, word: &str) {
        info!("Attempting to prime {}", word);
        let lock = self.suggestions.read().unwrap();
        if !lock.contains_key(word) {
            self.sender.send(word.to_string());
        }
    }

    pub fn is_correct(self: Arc<Self>, word: &str) -> bool {
        {
            let lock = self.is_correct.read().unwrap();
            let result = lock.get(word);
            if result.is_some() {
                return *result.unwrap();
            }
        }

        let is_correct = self.speller.to_owned().is_correct(word);
        self.is_correct.write().unwrap().insert(word.to_string(), is_correct);
        is_correct
    }

    pub fn suggest(self: Arc<Self>, word: &str) -> Vec<String> {
        {
            let lock = self.suggestions.read().unwrap();
            let result = lock.get(word);
            if result.is_some() {
                return result.unwrap().to_owned();
            }
        }

        
        let result = self.suggest_internal(word);
        self.suggestions.write().unwrap().insert(word.to_string(), result.to_owned());
        result
    }

    fn suggest_internal(self: &Arc<Self>, word: &str) -> Vec<String> {
        info!("suggest internal {}", word);
        let speller_config = SpellerConfig {
            n_best: Some(5),
            max_weight: Some(50.0),
            beam: None
        };
        
        let res: Vec<String> = self.speller.to_owned()
            .suggest_with_config(word, &speller_config)
            .iter().map(|s| s.value().to_string()).collect();
        
        info!("suggs {:?}", res.clone());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use hfstospell::archive::SpellerArchive;
        let archive = SpellerArchive::new(r"C:\Program Files\SpellCheckTest\dicts\se.zhfst").unwrap();
        let speller = archive.speller();

        let cache = SpellerCache::new(speller);
        println!("cache A {:?}", cache.to_owned().is_correct("hello"));
        println!("cache A2 {:?}", cache.to_owned().is_correct("hello"));
        println!("cache B {:?}", cache.to_owned().suggest("hello"));
        println!("cache B2 {:?}", cache.to_owned().suggest("hello"));
    }
}