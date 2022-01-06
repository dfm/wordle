use crate::{Strategy, Word};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum State {
    Gray(char),
    Yellow(char),
    Green(char),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rule<const SIZE: usize>([State; SIZE]);

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
        words
            .iter()
            .filter(|&word| self.check(word))
            .map(|&w| w)
            .collect()
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
        println!("Trying: {}", query);
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

    pub fn play<I, S>(&self, interface: &I, strategy: &S) -> Option<Word<SIZE>>
    where
        I: Interface<SIZE>,
        S: Strategy<SIZE>,
    {
        println!("initial dictionary size: {}", self.words.len());
        let mut filtered = if let Some(r) = self.first_rule {
            r.filter(&self.words)
        } else {
            self.words.clone()
        };
        while filtered.len() > 1 {
            println!("filtered dictionary size: {}", filtered.len());
            let query = strategy.select_query(&self.words, &filtered);
            let rule = interface.get_rule(&query);
            filtered = rule.filter(&filtered);
        }
        filtered.into_iter().next()
    }
}
