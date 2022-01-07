use clap::Parser;
use std::path::PathBuf;
use wordle::Interface;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// The path to your dictionary file
    #[clap(short, long, parse(from_os_str))]
    dictionary: Option<PathBuf>,

    /// Simulate a game with this test word
    #[clap(short, long)]
    simulate: Option<String>,

    /// Use hard mode
    #[clap(short, long)]
    hard: bool,
}

fn main() {
    let args = Args::parse();

    let words: Vec<wordle::Word<5>> = if let Some(path) = args.dictionary {
        wordle::load_words(path)
    } else {
        wordle::load_words("data/words_official.txt")
    };

    let strategy = wordle::Active;

    macro_rules! play_game {
        ($interface:expr) => {
            let interface = $interface;
            let rule = interface.get_rule(&"tares".into());
            let game = wordle::Game::new(&words, Some(rule));
            println!("{}", game.play(&interface, &strategy).unwrap());
        };
    }

    if let Some(word) = args.simulate {
        play_game!(wordle::Simulation::new(word.into()));
    } else {
        play_game!(wordle::UserInput);
    }
}
