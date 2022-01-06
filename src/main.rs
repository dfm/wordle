use wordle::Interface;

fn main() {
    let words: Vec<wordle::Word<5>> = wordle::load_words("data/words_official.txt");
    let strategy = wordle::Active;
    let interface = wordle::Simulation::new("sapid".into());
    // let interface = wordle::UserInput;
    let rule = interface.get_rule(&"tares".into());
    let game = wordle::Game::new(&words, Some(rule));
    println!("{}", game.play(&interface, &strategy).unwrap());
}
