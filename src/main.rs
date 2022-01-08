use clap::Parser;
use std::path::PathBuf;
use wordle::Interface;

/// Simulate a wordle game
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

    /// Don't print guesses
    #[clap(short, long)]
    quiet: bool,

    /// Run the baseline strategy
    #[clap(short, long)]
    baseline: bool,

    /// Optimize first guess
    #[clap(short, long)]
    first: bool,

    /// Use the cheat codes
    #[clap(short, long)]
    cheat: bool,
}

fn main() {
    let args = Args::parse();

    let corpus: Vec<wordle::Word<5>> = if let Some(path) = args.dictionary {
        wordle::load_words(path)
    } else {
        wordle::official_word_list()
    };

    let words = if args.cheat {
        wordle::official_cheat_list()
    } else {
        corpus.clone()
    };

    macro_rules! play_game {
        ($interface:expr) => {
            let mut interface = $interface;
            let game = wordle::Game::new(
                &corpus,
                &words,
                if args.first {
                    None
                } else {
                    Some(interface.get_rule(&(if args.cheat { "soare" } else { "tares" }).into()))
                },
            );
            if let Some(result) = if args.baseline {
                game.play(&mut interface, &wordle::Baseline, args.hard, args.quiet)
            } else {
                game.play(&mut interface, &wordle::Active, args.hard, args.quiet)
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
