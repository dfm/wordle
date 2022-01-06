use wordle::Interface;

fn main() {
    let words: Vec<wordle::Word<5>> = wordle::load_words("data/words_alpha.txt");
    let strategy = wordle::Active;
    let interface = wordle::Simulation::new("squid".into());
    // let interface = wordle::UserInput;
    let rule = interface.get_rule(&"tares".into());
    let game = wordle::Game::new(&words, Some(rule));
    game.play(&interface, &strategy);
}
