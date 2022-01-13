# A wordle simulator

A simulator for experimenting with different strategies for solving the [daily
Wordle puzzle](https://www.powerlanguage.co.uk/wordle/).

## Strategies

There are [2 strategies](src/strategy) implemented here: (1)
[`Baseline`](src/strategy/baseline.rs) a baseline that randomly guesses valid
words, and (2) [`Active`](src/strategy/active.rs) as algorithm that maximizes
the expected information gain at each step.

Simulating the performance of these algorithms for all words in the Wordle
dictionary gives the following results:

<img src="assets/figure.png" width=500 alt="The performance of our strategies">

where the orange histogram shows the distribution of guesses required when the
solver doesn't know that the correct Wordle is a "common" word. The green
distribution shows our results when we cheat and provide our algorithm with the
known word list from the Wordle source code.

## Usage

### Simulation mode

To simulate the performance for a specific puzzle, run:

```bash
cargo run -- -s array
```

where `array` can be replaced by [any valid 5-letter word](src/words.txt). This
will report results like:

```bash
tares: 12971 -> 80
monad: 80 -> 13
fugly: 13 -> 2
'array' in 4 guesses
```

This interface includes other command line arguments like `--hard` to run in
"hard mode" and `--common` to let the algorithm know that the true word is a
"common" word as defined by Wordle.

To run this simulation across the entire dictionary, run:

```bash
cargo run --bin simulate
```

This will print a comma-separated list or results to standard out, with the form:

```
cigar,4
rebut,3
sissy,4
humph,4
...
```

where the number is the number of guesses that were required to solve the
puzzle.

### Solve mode

You can also use this code to solve today's puzzle. The simplest way to do this
is by using the command line interface:

```bash
cargo run
```

which will suggest guesses and ask for you to enter the results from the website
by hand.

There is also a built-in solver that can interact with the website directly
using [thirtyfour](https://docs.rs/thirtyfour). To do this, you'll need to run a
Chrome WebDriver process using something like
[chromedriver](https://chromedriver.chromium.org/downloads):

```bash
chromedriver --url-base=/wd/hub --port=4444
```

Then run

```bash
cargo run --bin solve
```

which should show you something like the following:

![An example solve](assets/solve.gif)

This interface also takes command line arguments like `--hard` to solve in "hard mode":

```bash
cargo run --bin solve -- --hard
```
