use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod simulation;
use simulation::filter_word_list;

mod baseline;
pub use baseline::*;

mod active;
pub use active::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Word<const SIZE: usize>(pub [char; SIZE]);

impl<const SIZE: usize> std::fmt::Display for Word<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0.iter().map(|&c| c).collect::<String>())
    }
}

pub trait Strategy<const SIZE: usize> {
    fn select_query(&self, full_word_list: &[Word<SIZE>], valid_words: &[Word<SIZE>])
        -> Word<SIZE>;

    fn play(&self, query: &Word<SIZE>, words: &[Word<SIZE>], truth: &Word<SIZE>) {
        let mut filtered = filter_word_list(query, truth, words);
        println!("query: {}", query);
        println!("word list size: {}", filtered.len());
        while filtered.len() > 1 {
            let query = self.select_query(words, &filtered);
            filtered = filter_word_list(&query, truth, &filtered);
            println!("query: {}", query);
            println!("word list size: {}", filtered.len());
        }
        println!("{:?}", filtered);
    }
}

pub fn load_words<P: AsRef<Path>, const SIZE: usize>(path: P) -> Vec<Word<SIZE>> {
    let mut words = Vec::new();
    let file = File::open(path).unwrap();
    for line in io::BufReader::new(file).lines() {
        let chars = line
            .unwrap()
            .trim()
            .to_lowercase()
            .chars()
            .collect::<Vec<_>>();
        if chars.len() == SIZE {
            let mut word = Word(['a'; SIZE]);
            word.0[..SIZE].clone_from_slice(&chars);
            words.push(word);
        }
    }
    words
}
