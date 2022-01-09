use clap::Parser;
use rayon::prelude::*;
use std::path::PathBuf;
use wordle::{Simulation, StandardGame, StrategyType};

/// Simulate a wordle game
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// The path to your dictionary file
    #[clap(short, long, parse(from_os_str))]
    dictionary: Option<PathBuf>,

    /// Use hard mode
    #[clap(short, long)]
    hard: bool,

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
        .quiet(true)
        .common(args.common)
        .optimize_first_guess(args.first)
        .build();
    let strategy = if args.baseline {
        StrategyType::Baseline
    } else {
        StrategyType::Active
    };

    let results = game
        .possible_words
        .par_iter()
        .map(|&word| {
            let mut interface = Simulation::new(word.into());
            strategy.play_game(&mut interface, &game).unwrap()
        })
        .collect::<Vec<_>>();

    for &(count, word) in results.iter() {
        println!("{},{}", word, count);
    }
}
