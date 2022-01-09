use clap::Parser;
use std::path::PathBuf;
use wordle::{StandardGame, StrategyType};

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
    common: bool,
}

fn main() {
    let args = Args::parse();

    let game = StandardGame::new()
        .dictionary(args.dictionary)
        .hard(args.hard)
        .quiet(args.quiet)
        .common(args.common)
        .optimize_first_guess(args.first)
        .build();

    let strategy = if args.baseline {
        StrategyType::Baseline
    } else {
        StrategyType::Active
    };

    macro_rules! play_game {
        ($interface:expr) => {
            let mut interface = $interface;

            if let Some(result) = strategy.play_game(&mut interface, &game) {
                println!("'{}' in {} guesses", result.1, result.0);
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
