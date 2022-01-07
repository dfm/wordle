use crate::{Strategy, Word};
use colored::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum State {
    Gray(char),
    Yellow(char),
    Green(char),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rule<const SIZE: usize>([State; SIZE]);

impl<const SIZE: usize> std::fmt::Display for Rule<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = String::new();
        for s in self.0.iter() {
            result.push_str(&format!(
                "{}",
                match s {
                    State::Gray(c) => c.to_string().dimmed(),
                    State::Yellow(c) => c.to_string().yellow(),
                    State::Green(c) => c.to_string().green(),
                }
            ));
        }
        f.write_str(&result)
    }
}

impl<const SIZE: usize> Rule<SIZE> {
    pub fn from_mask(query: &Word<SIZE>, mask: &Word<SIZE>) -> Self {
        let mut rule = Self([State::Gray('a'); SIZE]);
        for ((&q, &m), s) in query.0.iter().zip(mask.0.iter()).zip(rule.0.iter_mut()) {
            match m {
                '0' => *s = State::Gray(q),
                '1' => *s = State::Yellow(q),
                '2' => *s = State::Green(q),
                _ => panic!(),
            }
        }
        rule
    }

    pub fn from_query(query: &Word<SIZE>, word: &Word<SIZE>) -> Self {
        let mut rule = Self([State::Gray('a'); SIZE]);
        for ((&q, &w), s) in query.0.iter().zip(word.0.iter()).zip(rule.0.iter_mut()) {
            if q == w {
                *s = State::Green(q);
            } else {
                if word.0.iter().any(|&w| w == q) {
                    *s = State::Yellow(q);
                } else {
                    *s = State::Gray(q);
                }
            }
        }
        rule
    }

    pub fn check(&self, word: &Word<SIZE>) -> bool {
        self.0.iter().zip(word.0.iter()).all(|(&s, &c)| match s {
            State::Gray(x) => word.0.iter().all(|&w| w != x),
            State::Yellow(x) => c != x && word.0.iter().any(|&w| w == x),
            State::Green(x) => c == x,
        })
    }

    pub fn filter(&self, words: &[Word<SIZE>]) -> Vec<Word<SIZE>> {
        let init_len = words.len();
        let result = words
            .iter()
            .filter(|&word| self.check(word))
            .map(|&w| w)
            .collect::<Vec<_>>();
        println!("{}: {} -> {}", self, init_len, result.len());
        result
    }
}

pub trait Interface<const SIZE: usize> {
    fn get_rule(&self, query: &Word<SIZE>) -> Rule<SIZE>;
}

pub struct Simulation<const SIZE: usize>(Word<SIZE>);
impl<const SIZE: usize> Simulation<SIZE> {
    pub fn new(truth: Word<SIZE>) -> Self {
        Self(truth)
    }
}
impl<const SIZE: usize> Interface<SIZE> for Simulation<SIZE> {
    fn get_rule(&self, query: &Word<SIZE>) -> Rule<SIZE> {
        Rule::from_query(query, &self.0)
    }
}

pub struct UserInput<const SIZE: usize>;
impl<const SIZE: usize> Interface<SIZE> for UserInput<SIZE> {
    fn get_rule(&self, query: &Word<SIZE>) -> Rule<SIZE> {
        println!("Try: {}", query);
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        Rule::from_mask(query, &buf.trim().into())
    }
}

pub struct Game<const SIZE: usize> {
    words: Vec<Word<SIZE>>,
    first_rule: Option<Rule<SIZE>>,
}

impl<const SIZE: usize> Game<SIZE> {
    pub fn new(words: &[Word<SIZE>], first_rule: Option<Rule<SIZE>>) -> Self {
        Self {
            words: words.into(),
            first_rule,
        }
    }

    pub fn play<I, S>(&self, interface: &I, strategy: &S, hard: bool) -> Option<Word<SIZE>>
    where
        I: Interface<SIZE>,
        S: Strategy<SIZE>,
    {
        let mut filtered = if let Some(r) = self.first_rule {
            r.filter(&self.words)
        } else {
            self.words.clone()
        };
        while filtered.len() > 1 {
            let query =
                strategy.select_query(if hard { &filtered } else { &self.words }, &filtered);
            let rule = interface.get_rule(&query);
            if rule.0.iter().all(|&s| matches!(s, State::Green(_))) {
                return Some(query);
            }
            filtered = rule.filter(&filtered);
        }
        filtered.into_iter().next()
    }
}
