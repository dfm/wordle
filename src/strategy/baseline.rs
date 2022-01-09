use crate::{strategy::Strategy, Word};
use rand::{seq::SliceRandom, thread_rng};

pub struct Baseline;
impl<const SIZE: usize> Strategy<SIZE> for Baseline {
    fn select_query(&self, _: &[Word<SIZE>], words: &[Word<SIZE>]) -> Word<SIZE> {
        let mut rng = thread_rng();
        *words.choose(&mut rng).unwrap()
    }
}
