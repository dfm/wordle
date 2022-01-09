use crate::{Game, Interface, Word};

mod active;
mod baseline;

pub trait Strategy<const SIZE: usize> {
    fn select_query(
        &self,
        allowed_queries: &[Word<SIZE>],
        possible_words: &[Word<SIZE>],
    ) -> Word<SIZE>;
}

pub enum StrategyType {
    Baseline,
    Active,
}

impl StrategyType {
    pub fn play_game<I: Interface<SIZE>, const SIZE: usize>(
        &self,
        interface: &mut I,
        game: &Game<SIZE>,
    ) -> Option<(usize, Word<SIZE>)> {
        match self {
            StrategyType::Baseline => game.play(interface, &baseline::Baseline),
            StrategyType::Active => game.play(interface, &active::Active),
        }
    }
}
