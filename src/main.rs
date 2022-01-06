use wordle::Strategy;

fn main() {
    let words: Vec<wordle::Word<5>> = wordle::load_words("data/words_alpha.txt");
    let query = wordle::Word(['t', 'a', 'r', 'e', 's']);
    let truth = wordle::Word(['t', 'i', 'g', 'e', 'r']);
    let strategy = wordle::Active;
    strategy.play(&query, &words, &truth);
}
