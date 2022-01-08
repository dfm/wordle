use crate::{Strategy, Word};
use colored::Colorize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum State {
    Gray(char),
    Yellow(char),
    Green(char),
}

#[must_use]
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
                _ => unreachable!("Invalid mask character; must be 0, 1, or 2"),
            }
        }
        rule
    }

    pub fn from_query(query: &Word<SIZE>, word: &Word<SIZE>) -> Self {
        let mut rule = Self([State::Gray('a'); SIZE]);
        for ((&q, &w), s) in query.0.iter().zip(word.0.iter()).zip(rule.0.iter_mut()) {
            if q == w {
                *s = State::Green(q);
            } else if word.0.iter().any(|&w| w == q) {
                *s = State::Yellow(q);
            } else {
                *s = State::Gray(q);
            }
        }
        rule
    }

    pub fn wins(&self) -> bool {
        self.0.iter().all(|&s| matches!(s, State::Green(_)))
    }

    pub fn check(&self, word: &Word<SIZE>) -> bool {
        self.0.iter().zip(word.0.iter()).all(|(&s, &c)| match s {
            State::Gray(x) => word.0.iter().all(|&w| w != x),
            State::Yellow(x) => c != x && word.0.iter().any(|&w| w == x),
            State::Green(x) => c == x,
        })
    }

    pub fn filter(&self, words: &[Word<SIZE>], quiet: bool) -> Vec<Word<SIZE>> {
        let init_len = words.len();
        let result = words
            .iter()
            .filter(|&word| self.check(word))
            .copied()
            .collect::<Vec<_>>();
        if !quiet {
            println!("{}: {} -> {}", self, init_len, result.len());
        }
        result
    }
}

pub trait Interface<const SIZE: usize> {
    fn get_rule(&mut self, query: &Word<SIZE>) -> Rule<SIZE>;
}

pub struct Simulation<const SIZE: usize>(Word<SIZE>);
impl<const SIZE: usize> Simulation<SIZE> {
    pub fn new(truth: Word<SIZE>) -> Self {
        Self(truth)
    }
}
impl<const SIZE: usize> Interface<SIZE> for Simulation<SIZE> {
    fn get_rule(&mut self, query: &Word<SIZE>) -> Rule<SIZE> {
        Rule::from_query(query, &self.0)
    }
}

pub struct UserInput<const SIZE: usize>;
impl<const SIZE: usize> Interface<SIZE> for UserInput<SIZE> {
    fn get_rule(&mut self, query: &Word<SIZE>) -> Rule<SIZE> {
        use std::io::Write;
        println!("Try: {}", query.to_string().bold());
        print!(
            "Result ({}, {}, {}; e.g. {}{}{}{}{}): ",
            "0: gray".dimmed(),
            "1: yellow".yellow(),
            "2: green".green(),
            "0".dimmed(),
            "1".yellow(),
            "0".dimmed(),
            "2".green(),
            "1".yellow(),
        );
        let _ = std::io::stdout().flush();
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Input required");
        Rule::from_mask(query, &buf.trim().into())
    }
}

pub struct Game<const SIZE: usize> {
    corpus: Vec<Word<SIZE>>,
    allowed: Vec<Word<SIZE>>,
    first_rule: Option<Rule<SIZE>>,
}

impl<const SIZE: usize> Game<SIZE> {
    pub fn new(
        corpus: &[Word<SIZE>],
        allowed: &[Word<SIZE>],
        first_rule: Option<Rule<SIZE>>,
    ) -> Self {
        Self {
            corpus: corpus.into(),
            allowed: allowed.into(),
            first_rule,
        }
    }

    pub fn play<I, S>(
        &self,
        interface: &mut I,
        strategy: &S,
        hard: bool,
        quiet: bool,
    ) -> Option<Word<SIZE>>
    where
        I: Interface<SIZE>,
        S: Strategy<SIZE>,
    {
        let mut words = self.corpus.clone();
        let mut filtered = if let Some(r) = self.first_rule {
            if hard {
                words = r.filter(&words, true);
            }
            r.filter(&self.allowed, quiet)
        } else {
            self.allowed.clone()
        };
        while filtered.len() > 1 {
            let query = strategy.select_query(&words, &filtered);
            let rule = interface.get_rule(&query);
            if rule.wins() {
                return Some(query);
            }
            if hard {
                words = rule.filter(&words, true);
            }
            filtered = rule.filter(&filtered, quiet);
        }
        let result = filtered.into_iter().next();
        result.and_then(|word| {
            let rule = interface.get_rule(&word);
            if rule.wins() {
                Some(word)
            } else {
                None
            }
        })
    }
}
