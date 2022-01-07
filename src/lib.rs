use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod game;
pub use game::*;

mod baseline;
pub use baseline::*;

mod active;
pub use active::*;

mod counter;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Word<const SIZE: usize>(pub [char; SIZE]);

impl<const SIZE: usize> Word<SIZE> {
    pub fn from_string(s: &str) -> Self {
        let mut word = Word(['a'; SIZE]);
        word.0[..SIZE].clone_from_slice(&s.chars().collect::<Vec<_>>());
        word
    }
}

impl<const SIZE: usize> From<&str> for Word<SIZE> {
    fn from(s: &str) -> Self {
        Word::from_string(s)
    }
}

impl<const SIZE: usize> From<String> for Word<SIZE> {
    fn from(s: String) -> Self {
        Word::from_string(&s)
    }
}

impl<const SIZE: usize> std::fmt::Display for Word<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0.iter().copied().collect::<String>())
    }
}

pub trait Strategy<const SIZE: usize> {
    fn select_query(&self, full_word_list: &[Word<SIZE>], valid_words: &[Word<SIZE>])
        -> Word<SIZE>;
}

pub fn load_words<P: AsRef<Path>, const SIZE: usize>(path: P) -> Vec<Word<SIZE>> {
    let mut words = Vec::new();
    let file = File::open(path).unwrap();
    for line in io::BufReader::new(file).lines() {
        let chars = line.unwrap().trim().to_lowercase();
        if chars.len() == SIZE {
            words.push(chars.into());
        }
    }
    words
}

pub fn official_word_list() -> Vec<Word<5>> {
    std::include_str!("words.txt")
        .lines()
        .map(|l| l.trim().into())
        .collect()
}
