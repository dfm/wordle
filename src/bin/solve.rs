use clap::Parser;
use thirtyfour_sync::{
    http::reqwest_sync::ReqwestDriverSync, prelude::*, ElementId, GenericWebDriver,
};
use wordle::{Interface, Rule, StandardGame, StrategyType, Word};

/// Simulate a wordle game
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
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

    /// Don't print progress
    #[clap(short, long)]
    quiet: bool,
}

#[derive(serde::Deserialize)]
pub struct ShadowElementRef {
    #[serde(rename(deserialize = "shadow-6066-11e4-a52e-4f735466cecf"))]
    pub id: String,
}

struct WebInput<'a, const SIZE: usize> {
    driver: &'a GenericWebDriver<ReqwestDriverSync>,
    game: &'a WebElement<'a>,
    row: usize,
    rules: Vec<Rule<SIZE>>,
}

impl<'a, const SIZE: usize> Interface<SIZE> for WebInput<'a, SIZE> {
    fn get_rule(&mut self, query: &Word<SIZE>) -> Rule<SIZE> {
        std::thread::sleep(std::time::Duration::from_secs(2));

        let keyboard = self.game.query(By::Tag("game-keyboard")).first().unwrap();
        let keyboard = shadow_root(&self.driver, &keyboard).unwrap();

        macro_rules! press_key {
            ($key:expr) => {
                keyboard
                    .query(By::Css(&format!("button[data-key='{}']", $key)))
                    .first()
                    .unwrap()
                    .click()
                    .unwrap();
            };
        }

        for &c in query.0.iter() {
            press_key!(c);
        }
        press_key!("↵");

        let rows = self.game.query(By::Tag("game-row")).all().unwrap();
        let row = shadow_root(&self.driver, &rows[self.row]).unwrap();

        let mut tiles = Vec::new();
        while tiles.len() < 5 {
            tiles = row
                .query(By::Css("game-tile"))
                .with_attribute("reveal", "")
                .all()
                .unwrap();
        }
        let mut mask = Word(['0'; SIZE]);
        for (tile, m) in tiles.iter().zip(mask.0.iter_mut()) {
            *m = match tile.get_attribute("evaluation").unwrap().unwrap().as_str() {
                "present" => '1',
                "correct" => '2',
                _ => '0',
            };
        }
        self.row += 1;
        let rule = Rule::from_mask(query, &mask);
        self.rules.push(rule.clone());
        rule
    }
}

fn shadow_root<'a>(
    driver: &'a GenericWebDriver<ReqwestDriverSync>,
    element: &'a WebElement<'a>,
) -> WebDriverResult<WebElement<'a>> {
    let mut args = ScriptArgs::new();
    args.push(element.clone())?;
    let ret = driver.execute_script_with_args("return arguments[0].shadowRoot;", &args)?;
    let elem_id: ShadowElementRef = serde_json::from_value(ret.value().clone())?;
    Ok(WebElement::new(
        &element.session,
        ElementId::from(elem_id.id),
    ))
}

fn main() -> WebDriverResult<()> {
    // println!("hi: 🟩🟨⬛");
    // Ok(())

    let args = Args::parse();
    let game = StandardGame::new()
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

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)?;

    // Navigate to URL.
    driver.get("https://www.powerlanguage.co.uk/wordle/")?;
    let game_elem = driver.query(By::Tag("game-app")).first()?;
    let game_elem = shadow_root(&driver, &game_elem)?;

    // Close the popup
    let modal = game_elem.query(By::Tag("game-modal")).first()?;
    let mut args = ScriptArgs::new();
    args.push(modal.clone())?;
    driver.execute_script_with_args("arguments[0].remove()", &args)?;

    // Play the game
    let mut interface = WebInput::<5> {
        driver: &driver,
        game: &game_elem,
        row: 0,
        rules: Vec::new(),
    };
    let result = strategy.play_game(&mut interface, &game).unwrap();
    println!("'{}' in {} guesses", result.1, result.0);

    std::thread::sleep(std::time::Duration::from_secs(10));

    // Close the browser.
    driver.quit()?;

    Ok(())
}
