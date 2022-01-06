use super::{Strategy, Word};

pub struct Baseline;
impl<const SIZE: usize> Strategy<SIZE> for Baseline {
    fn select_query(&self, _: &[Word<SIZE>], words: &[Word<SIZE>]) -> Word<SIZE> {
        let mut count = [[0; 26]; SIZE];
        for &word in words.iter() {
            for (n, &c) in word.0.iter().enumerate() {
                count[n][(c as u8 - b'a') as usize] += 1;
            }
        }
        let mut query = Word(['a'; SIZE]);
        for n in 0..SIZE {
            query.0[n] = ('a'..='z')
                .filter(|&c| query.0.iter().take(n).all(|&x| x != c))
                .max_by_key(|&c| count[n][(c as u8 - b'a') as usize])
                .unwrap();
        }
        query
    }
}
