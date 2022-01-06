use super::{Strategy, Word};
use std::collections::HashMap;

const ANYTHING: char = 'A';

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Rule<const SIZE: usize> {
    pos_mask: Word<SIZE>,
    neg_mask: Word<SIZE>,
    require: [bool; 26],
    disallow: [bool; 26],
}

pub trait Counter<Key: std::hash::Hash + Eq> {
    fn counter_from_iter(iter: impl Iterator<Item = Key>) -> Self;
    fn increment(&mut self, key: Key, amount: usize);
}

impl<Key: std::hash::Hash + Eq> Counter<Key> for HashMap<Key, usize> {
    fn counter_from_iter(iter: impl Iterator<Item = Key>) -> Self {
        let mut counter = Self::new();
        for item in iter {
            counter.increment(item, 1);
        }
        counter
    }

    fn increment(&mut self, key: Key, amount: usize) {
        let target = self.entry(key).or_insert(0);
        *target += amount;
    }
}

impl<const SIZE: usize> Rule<SIZE> {
    fn new(query: &Word<SIZE>, word: &Word<SIZE>) -> Self {
        let mut rule = Self {
            pos_mask: Word([ANYTHING; SIZE]),
            neg_mask: Word([ANYTHING; SIZE]),
            require: [false; 26],
            disallow: [false; 26],
        };
        for ((&q, &w), (pos, neg)) in query
            .0
            .iter()
            .zip(word.0.iter())
            .zip(rule.pos_mask.0.iter_mut().zip(rule.neg_mask.0.iter_mut()))
        {
            if q == w {
                *pos = q;
            } else {
                *neg = q;
            }
        }
        for &q in query.0.iter() {
            if word.0.iter().any(|&w| w == q) {
                rule.require[(q as u8 - b'a') as usize] = true;
            } else {
                rule.disallow[(q as u8 - b'a') as usize] = true;
            }
        }
        rule
    }
}

#[derive(PartialEq)]
struct Entropy(f64);
impl std::cmp::PartialOrd for Entropy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::cmp::Eq for Entropy {}

impl std::cmp::Ord for Entropy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn entropy<'a, T: Iterator<Item = &'a usize>>(iter: T) -> Entropy {
    Entropy(
        iter.map(|&n| {
            let n = n as f64;
            n * n.ln()
        })
        .sum(),
    )
}

pub struct Active;
impl<const SIZE: usize> Strategy<SIZE> for Active {
    fn select_query(
        &self,
        full_word_list: &[Word<SIZE>],
        valid_words: &[Word<SIZE>],
    ) -> Word<SIZE> {
        if valid_words.len() <= 2 {
            return valid_words[0];
        }
        *full_word_list
            .iter()
            .min_by_key(|&query| {
                let counter = HashMap::counter_from_iter(
                    valid_words.iter().map(|word| Rule::new(&query, word)),
                );
                entropy(counter.values())
            })
            .unwrap()
    }
}
