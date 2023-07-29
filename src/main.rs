use std::error::Error;

use lazy_static::lazy_static;
use path_absolutize::Absolutize;
use scraper::Html;

use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::config::Config;
use booth_archiver::models::web_scrapper::WebScraper;
use booth_archiver::time_it;

lazy_static! {
    pub static ref CONFIG: Config = time_it!("loading config" => Config::get());
    pub static ref COOKIE: String = {
        std::fs::read_to_string(
            CONFIG
                .cookie_file
                .as_ref()
                .expect("failed to build Path from PathBuf"),
        )
        .unwrap_or_else(|e| {
            panic!(
                "expecting cookie to be in {}, because of error: {}",
                CONFIG
                    .cookie_file
                    .as_ref()
                    .expect(
                        "failed to build Path from PathBuf, \
                        imagine panicking inside a panic lmao"
                    )
                    .absolutize()
                    .expect(
                        "failed to absolutize path from PathBuf, \
                        imagine panicking inside a panic lmao"
                    )
                    .to_str()
                    .expect("failed to convert path to str, imagine panicking inside a panic lmao"),
                e
            )
        })
    };
}

// the reason we cannot parallelize this is because Document is not Send,
// and we cannot send it to another thread
fn main() -> Result<(), Box<dyn Error>> {
    let client = WebScraper::new(COOKIE.to_string(), true);

    let wishlist_pages = get_all_wishlist_pages(&client)?;

    let mut all_item_numbers = Vec::new();
    for page in wishlist_pages {
        let page = Html::parse_document(&page);
        let item_numbers = get_all_item_numbers_on_page(&page)?;
        all_item_numbers.extend(item_numbers);
    }

    println!("number of items = {}", all_item_numbers.len());

    let all_items = get_items(&client, all_item_numbers)?;

    time_it!("dumping" => client.dump_cache());

    println!("number of items: {:?}", all_items.len());

    println!("{}", client.get_cache_stats());

    Ok(())
}
