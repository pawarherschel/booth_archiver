use std::error::Error;

use lazy_static::lazy_static;

use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::config::Config;
use booth_archiver::models::web_scrapper::WebScraper;

lazy_static! {
    pub static ref CONFIG: Config = {
        println!("Config::get()");
        Config::get()
    };
    pub static ref COOKIE: String = {
        println!("cookie_file: {:?}", CONFIG.cookie_file.as_ref().unwrap());
        std::fs::read_to_string(CONFIG.cookie_file.as_ref().unwrap()).unwrap()
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = WebScraper::new(COOKIE.to_string(), true).await;

    let wishlist_pages = get_all_wishlist_pages(&client).await?;

    let mut all_item_numbers = Vec::new();
    for page in wishlist_pages {
        let item_numbers = get_all_item_numbers_on_page(&page).await?;
        all_item_numbers.extend(item_numbers);
    }

    println!(
        "all_item_numbers: {:?}\nTotal = {}",
        all_item_numbers,
        all_item_numbers.len()
    );

    // let all_items = time_it!("getting all pages" =>
    //     get_items(&client, all_item_numbers).await?);
    //
    // println!("all_items: {:?}", all_items.len());

    Ok(())
}
