use super::Word;

fn check<const SIZE: usize>(query: &Word<SIZE>, truth: &Word<SIZE>, word: &Word<SIZE>) -> bool {
    if word
        .0
        .iter()
        .zip(truth.0.iter().zip(query.0.iter()))
        .any(|(&w, (&t, &q))| t == q && w != q)
    {
        return false;
    }

    for (&w, (&t, &q)) in word.0.iter().zip(truth.0.iter().zip(query.0.iter())) {
        if t != q {
            // Query doesn't match truth
            let flag = word.0.iter().any(|&x| x == q);
            if truth.0.iter().any(|&x| x == q) {
                if w == q || !flag {
                    // 'q' must appear in 'word' but not at this location
                    return false;
                }
            } else if flag {
                // 'q' must not appear in 'word'
                return false;
            }
        }
    }
    true
}

pub fn filter_word_list<const SIZE: usize>(
    query: &Word<SIZE>,
    truth: &Word<SIZE>,
    words: &[Word<SIZE>],
) -> Vec<Word<SIZE>> {
    words
        .iter()
        .filter_map(|&word| {
            if check(query, truth, &word) {
                Some(word)
            } else {
                None
            }
        })
        .collect()
}
