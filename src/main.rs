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

    /// Run the baseline strategy
    #[clap(short, long)]
    baseline: bool,
}

fn main() {
    let args = Args::parse();

    let words: Vec<wordle::Word<5>> = if let Some(path) = args.dictionary {
        wordle::load_words(path)
    } else {
        wordle::official_word_list()
    };

    macro_rules! play_game {
        ($interface:expr) => {
            let interface = $interface;
            let rule = interface.get_rule(&"tares".into());
            let game = wordle::Game::new(&words, Some(rule));
            if let Some(result) = if args.baseline {
                game.play(&interface, &wordle::Baseline, args.hard)
            } else {
                game.play(&interface, &wordle::Active, args.hard)
            } {
                println!("{}", result);
            } else {
                println!("There aren't any words in the word list that satisfy these constraints")
            }
        };
    }

    if let Some(word) = args.simulate {
        play_game!(wordle::Simulation::new(word.into()));
    } else {
        play_game!(wordle::UserInput);
    }
}
