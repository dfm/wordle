use crate::{counter::Counter, Rule, Strategy, Word};
use std::collections::HashMap;

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
                    valid_words.iter().map(|word| Rule::from_query(query, word)),
                );
                expected_entropy(counter.values())
            })
            .unwrap()
    }
}

fn expected_entropy<'a, T: Iterator<Item = &'a usize>>(iter: T) -> Entropy {
    Entropy(
        iter.map(|&n| {
            let n = n as f64;
            n * n.ln()
        })
        .sum(),
    )
}

/// A type to encapsulate the entropy to deal with the fact that we can't sort
/// the f64 data type.
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
