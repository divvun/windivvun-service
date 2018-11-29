use std::collections::{HashSet, HashMap};
use parking_lot::RwLock;
use std::sync::Arc;
use std::iter::FromIterator;

#[derive(Hash)]
struct AutoCorrection(String, String);

pub struct Wordlists {
    ignore: RwLock<HashSet<String>>,
    add: RwLock<HashSet<String>>,
    exclude: RwLock<HashSet<String>>,
    auto_correct: RwLock<HashMap<String, String>>
}

impl Wordlists {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            ignore: RwLock::new(HashSet::new()),
            add: RwLock::new(HashSet::new()),
            exclude: RwLock::new(HashSet::new()),
            auto_correct: RwLock::new(HashMap::new()),
        })
    }

    pub fn contains_add(self: &Arc<Self>, word: &str) -> bool {
        self.add.read().contains(word)
    }

    pub fn contains_ignore(self: &Arc<Self>, word: &str) -> bool {
        self.ignore.read().contains(word)
    }

    pub fn contains_exclude(self: &Arc<Self>, word: &str) -> bool {
        self.exclude.read().contains(word)
    }

    pub fn get_replacement(self: &Arc<Self>, word: &str) -> Option<String> {
        self.auto_correct.read().get(word).cloned()
    }

    pub fn set_ignore(self: &Arc<Self>, words: Vec<String>) {
        let mut list = self.ignore.write();
        list.clear();
        for word in words { list.insert(word.to_string()); }
    }

    pub fn set_add(self: &Arc<Self>, words: Vec<String>) {
        let mut list = self.add.write();
        list.clear();
        for word in words { list.insert(word.to_string()); }
    }

    pub fn set_exclude(self: &Arc<Self>, words: Vec<String>) {
        let mut list = self.exclude.write();
        list.clear();
        for word in words { list.insert(word.to_string()); }
    }

    pub fn set_auto_correct(self: &Arc<Self>, words: HashMap<String, String>) {
        let mut map = self.auto_correct.write();
        map.clear();
        for (k, v) in words {
            map.insert(k, v);
        }
    }
}