use dashmap::DashMap;
use indexmap::IndexMap;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static DEFAULT_IGNORE: &[&str] = &[
    "the", "and", "a", "to", "of", "in", "i", "it", "is", "on", "that", "you", "this", "for",
];

pub trait WordCounter {
    fn increment(&mut self, key: &str);
}

impl WordCounter for DashMap<String, usize> {
    fn increment(&mut self, key: &str) {
        self.entry(key.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}

impl WordCounter for IndexMap<String, usize> {
    fn increment(&mut self, key: &str) {
        self.entry(key.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}

impl WordCounter for HashMap<String, usize> {
    fn increment(&mut self, key: &str) {
        self.entry(key.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}

pub fn count_words<T>(
    content: &str,
    ignore_option: &str,
    custom_ignore_words: &[&str],
    final_word_counts: &mut T,
) where
    T: WordCounter + Send + Sync,
{
    let ignored: HashSet<String> = if ignore_option == "CUSTOM_IGNORED" {
        custom_ignore_words
            .iter()
            .cloned()
            .map(String::from)
            .collect()
    } else {
        DEFAULT_IGNORE.iter().cloned().map(String::from).collect()
    };

    let re = Regex::new(r"[^a-zA-Z0-9']+").unwrap();
    let processed_content = re.replace_all(content, " ");
    let words = processed_content.split_whitespace();

    for word in words {
        let word = word.to_lowercase();
        if !word.is_empty() && !ignored.contains(&word) {
            final_word_counts.increment(&word);
        }
    }
}
