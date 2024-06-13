use crate::word_counter::{count_words, WordCounter};

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::Arc;

const CHUNK_SIZE: usize = 6 * 1024 * 1024; // 6MB

pub fn read_input_file(path: &PathBuf) -> io::Result<String> {
    println!("Attempting to read file: {:?}", path); // 打印文件路径
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_multiple_files_from_dir(directory: &str) -> io::Result<Vec<Arc<Vec<u8>>>> {
    let mut all_chunks = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("Found file: {:?}", path); // 打印发现的文件路径
            match file_splitter(&path) {
                // 调用 file_splitter 以切分文件
                Ok(chunks) => all_chunks.extend(chunks),
                Err(e) => return Err(e),
            }
        }
    }
    Ok(all_chunks)
}

pub fn file_splitter(path: &PathBuf) -> io::Result<Vec<Arc<Vec<u8>>>> {
    let mut file = File::open(path)?;
    let mut chunks = Vec::new();

    loop {
        let mut chunk = vec![0; CHUNK_SIZE];
        let bytes_read = file.read(&mut chunk)?;

        if bytes_read == 0 {
            break;
        }

        chunk.truncate(bytes_read);
        chunks.push(Arc::new(chunk));
    }

    Ok(chunks)
}

pub fn process_files<T>(
    directory: &str,
    ignore_option: &str,
    custom_ignore_words: &[&str],
    final_word_counts: Arc<std::sync::Mutex<T>>,
) -> io::Result<()>
where
    T: WordCounter + Send + Sync,
{
    let files = read_multiple_files_from_dir(directory)?;

    for file in files {
        let content = String::from_utf8_lossy(&file);
        let mut counts = final_word_counts.lock().unwrap();
        count_words(&content, ignore_option, custom_ignore_words, &mut counts);
    }

    Ok(())
}
