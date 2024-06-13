mod utils;
mod word_counter;
use crate::utils::file_utils::process_files;
use dashmap::DashMap;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    let directory = "data/input";
    let ignore_option = "CUSTOM_IGNORED";
    let custom_ignore_words = vec!["example", "test"];

    // 使用 Arc 和 Mutex 包装 final_word_counts 以确保线程安全
    let final_word_counts: Arc<Mutex<DashMap<String, usize>>> =
        Arc::new(Mutex::new(DashMap::new()));
    // 或者使用 IndexMap 或 HashMap
    // 或者使用 IndexMap 或 HashMap
    // let final_word_counts = Arc::new(Mutex::new(IndexMap::new()));
    // let final_word_counts = Arc::new(Mutex::new(HashMap::new()));

    match process_files(
        directory,
        ignore_option,
        &custom_ignore_words,
        final_word_counts.clone(),
    ) {
        Ok(_) => {
            let counts = final_word_counts.lock().unwrap();
            for word_count in counts.iter() {
                println!("{}: {}", word_count.key(), word_count.value());
            }
        }
        Err(e) => println!("Failed to process files: {}", e),
    }
}
