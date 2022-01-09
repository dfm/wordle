use crate::{common_word_list, load_dictionary, official_word_list, strategy::Strategy, Word};
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
    hard: bool,
    quiet: bool,
    first_guess: Option<Word<SIZE>>,
    allowed_queries: Vec<Word<SIZE>>,
    pub possible_words: Vec<Word<SIZE>>,
}

impl<const SIZE: usize> Game<SIZE> {
    pub fn new(
        hard: bool,
        quiet: bool,
        first_guess: Option<Word<SIZE>>,
        allowed_queries: &[Word<SIZE>],
        possible_words: &[Word<SIZE>],
    ) -> Self {
        Self {
            hard,
            quiet,
            first_guess,
            allowed_queries: allowed_queries.into(),
            possible_words: possible_words.into(),
        }
    }

    pub fn play<I, S>(&self, interface: &mut I, strategy: &S) -> Option<(usize, Word<SIZE>)>
    where
        I: Interface<SIZE>,
        S: Strategy<SIZE>,
    {
        let mut count = 0;
        let mut allowed_queries = self.allowed_queries.clone();
        let mut possible_words = if let Some(w) = self.first_guess {
            let r = interface.get_rule(&w);
            if self.hard {
                allowed_queries = r.filter(&allowed_queries, true);
            }
            count += 1;
            r.filter(&self.possible_words, self.quiet)
        } else {
            self.possible_words.clone()
        };
        while possible_words.len() > 1 {
            count += 1;
            let query = strategy.select_query(&allowed_queries, &possible_words);
            let rule = interface.get_rule(&query);
            if rule.wins() {
                return Some((count, query));
            }
            if self.hard {
                allowed_queries = rule.filter(&allowed_queries, true);
            }
            possible_words = rule.filter(&possible_words, self.quiet);
        }
        let result = possible_words.into_iter().next();
        result.and_then(|word| {
            let rule = interface.get_rule(&word);
            if rule.wins() {
                Some((count + 1, word))
            } else {
                None
            }
        })
    }
}

#[must_use]
#[derive(Default)]
pub struct StandardGame {
    dictionary: Option<std::path::PathBuf>,
    hard: bool,
    quiet: bool,
    common: bool,
    optimize_first_guess: bool,
}

impl StandardGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dictionary(mut self, dictionary: Option<std::path::PathBuf>) -> Self {
        self.dictionary = dictionary;
        self
    }

    pub fn hard(mut self, hard: bool) -> Self {
        self.hard = hard;
        self
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn common(mut self, common: bool) -> Self {
        self.common = common;
        self
    }

    pub fn optimize_first_guess(mut self, optimize: bool) -> Self {
        self.optimize_first_guess = optimize;
        self
    }

    pub fn build(self) -> Game<5> {
        let allowed_queries = if let Some(path) = self.dictionary {
            load_dictionary(path)
        } else {
            official_word_list()
        };

        let possible_words = if self.common {
            common_word_list()
        } else {
            allowed_queries.clone()
        };

        let first_guess = if self.optimize_first_guess {
            None
        } else {
            Some((if self.common { "soare" } else { "tares" }).into())
        };

        Game::new(
            self.hard,
            self.quiet,
            first_guess,
            &allowed_queries,
            &possible_words,
        )
    }
}
