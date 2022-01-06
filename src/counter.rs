use std::collections::HashMap;

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
