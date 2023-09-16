use fs::File;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::Instant;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use rust_xlsxwriter::Workbook;
use scraper::Html;

use booth_archiver::api_structs::items::ItemApiResponse;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::time_it;
use booth_archiver::zaphkiel::cache::Cache;
use booth_archiver::zaphkiel::lazy_statics::*;
use booth_archiver::zaphkiel::utils::get_pb;
use booth_archiver::zaphkiel::xlsx::{save_book, write_all, write_headers};

fn main() {
    let start: Instant = Instant::now();

    let (wishlist_pages, _last_page_changed) = time_it!(at once | "getting wishlist pages" => {
            let (pages, changed) = get_all_wishlist_pages(&CLIENT);
            println!("number of pages = {}", pages.len());
            (pages, changed)
        }
    );

    let all_item_numbers = time_it!(at once | "extracting item numbers from pages" => {
        let mut all_item_numbers = Vec::new();
        for page in wishlist_pages {
            let page = Html::parse_document(&page);
            let item_numbers = get_all_item_numbers_on_page(&page);
            all_item_numbers.extend(item_numbers);
        }
        all_item_numbers
    });

    let mut path_to_cache = PathBuf::new();
    path_to_cache.push("cache");
    path_to_cache.push("all_items.ron");

    let cache = Arc::new(RwLock::new(Cache::new_with_path(path_to_cache)));

    let all_items = time_it!(at once | "Extracting items" => all_item_numbers
        // .iter()
        .par_iter()
        .progress_with(get_pb(all_item_numbers.len() as u64, "Extracting Items"))
        .map(|id| format!("https://booth.pm/en/items/{}.json", id))
        .filter_map(|url| CLIENT.get_one(url, Some(cache.clone())).ok())
        .filter_map(|item| serde_json::from_str::<ItemApiResponse>(&item).ok())
        .collect::<Vec<ItemApiResponse>>()
    );

    println!("all_items.len(): {}", all_items.len());

    time_it!(at once | "writing items to json file" => {
        let output_path = Path::new("temp/items.json");

        let mut file = File::create(output_path).unwrap();
        let all_items_pretty = serde_json::to_string_pretty(&all_items).unwrap();
        file.write_all(all_items_pretty.as_bytes()).unwrap();
    });

    time_it!(at once | "writing items to ron file" => {
        let output_path = Path::new("temp/items.ron");

        let mut file = File::create(output_path).unwrap();
        let all_items_pretty = ron::ser::to_string_pretty(&all_items, Default::default()).unwrap();
        file.write_all(all_items_pretty.as_bytes()).unwrap();
    });

    time_it!(at once | "writing items to xlsx" => {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        write_headers(worksheet).unwrap();

        write_all(worksheet, all_items);

        save_book(&mut workbook, "temp/book.xlsx");
    });

    time_it!("dumping" => cache.clone().write().unwrap().dump());
    println!("{:#?}", cache.clone().read().unwrap().get_stats());
    println!(
        "cache misses: {:#?}",
        cache.clone().read().unwrap().get_misses()
    );
    println!("time taken: {:#?}", start.elapsed());
}
