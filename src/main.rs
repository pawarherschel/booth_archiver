use fs::File;
use std::fs;
use std::io::Write;

use rayon::prelude::*;
use scraper::Html;

use booth_archiver::api_structs::items::Root;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::web_scrapper::WebScraper;
use booth_archiver::time_it;
use booth_archiver::zaphkiel::lazy_statics::*;
use booth_archiver::zaphkiel::utils::{
    check_if_the_unneeded_files_are_generated_and_panic_if_they_do, unneeded_values,
};

/// TODO: make it so the cache location is separate for every class of url, eg: wishlist pages, item pages, images, etc.
/// TODO: cache the images
/// TODO: use the fucking api dumbass: format!("https://booth.pm/en/items/{}.json", item_id)
fn main() {
    let start = std::time::Instant::now();
    let client = time_it!(at once | "creating client" =>
        WebScraper::new(COOKIE.to_string(), true)
    );

    let wishlist_pages = time_it!(at once | "getting wishlist pages" =>{
            let pages = get_all_wishlist_pages(&client);
            println!("number of pages = {}", pages.len());
            pages
        }
    );

    let all_item_numbers = time_it!(at once | "extracting items from pages" => {
        let mut all_item_numbers = Vec::new();
        for page in wishlist_pages {
            let page = Html::parse_document(&page);
            let item_numbers = get_all_item_numbers_on_page(&page);
            all_item_numbers.extend(item_numbers);
        }
        all_item_numbers
    });

    let all_items_json_url = all_item_numbers
        .iter()
        .map(|id| format!("https://booth.pm/en/items/{}.json", id))
        .collect::<Vec<_>>();

    let all_item_json = time_it!(at once | "getting all items json" => {
        client.get_many(all_items_json_url).iter().flatten().map(|s|s.to_owned()).collect::<Vec<_>>()
    });

    let mut errors = Vec::new();
    let mut error_json = Vec::new();

    let all_items = all_item_json
        .iter()
        .filter_map(|s| match serde_json::from_str::<Root>(s) {
            Ok(root) => Some(root),
            Err(e) => {
                errors.push(e.to_string());
                error_json.push(s.to_owned());
                None
            }
        })
        .collect::<Vec<_>>();

    println!("number of successes: {}", all_items.len());
    println!("number of errors: {}", errors.len());

    if !errors.is_empty() {
        let mut error_file = File::create("errors.json").unwrap();
        let mut error_json = error_json.join(",");
        error_json.insert(0, '[');
        error_json.push(']');
        error_file.write_all(error_json.as_bytes()).unwrap();
    }

    unneeded_values(&all_items);
    check_if_the_unneeded_files_are_generated_and_panic_if_they_do();

    time_it!("dumping" => client.dump_cache());
    println!("{}", client.get_cache_stats());
    println!("cache misses: {:}", client.get_cache_misses());
    println!("time taken: {:?}", start.elapsed());
}
