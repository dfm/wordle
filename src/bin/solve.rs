use thirtyfour_sync::{
    http::reqwest_sync::ReqwestDriverSync, prelude::*, ElementId, GenericWebDriver,
};
use wordle::{Interface, Rule, Word};

#[derive(serde::Deserialize)]
pub struct ShadowElementRef {
    #[serde(rename(deserialize = "shadow-6066-11e4-a52e-4f735466cecf"))]
    pub id: String,
}

struct WebInput<'a, const SIZE: usize> {
    driver: &'a GenericWebDriver<ReqwestDriverSync>,
    game: &'a WebElement<'a>,
    row: usize,
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
        Rule::from_mask(query, &mask)
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
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps)?;

    // Navigate to URL.
    driver.get("https://www.powerlanguage.co.uk/wordle/")?;
    let game = driver.query(By::Tag("game-app")).first()?;
    let game = shadow_root(&driver, &game)?;

    // Close the popup
    let modal = game.query(By::Tag("game-modal")).first()?;
    let mut args = ScriptArgs::new();
    args.push(modal.clone())?;
    driver.execute_script_with_args("arguments[0].remove()", &args)?;

    let words = wordle::official_word_list();
    let mut interface = WebInput::<5> {
        driver: &driver,
        game: &game,
        row: 0,
    };
    let rule = interface.get_rule(&"tares".into());
    let game = wordle::Game::new(&words, Some(rule));
    let result = game.play(&mut interface, &wordle::Active, false);
    let _ = interface.get_rule(&result.unwrap());
    println!("{}", result.unwrap());

    std::thread::sleep(std::time::Duration::from_secs(10));

    // Close the browser.
    driver.quit()?;

    Ok(())
}
