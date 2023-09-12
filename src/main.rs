use fs::File;
use std::fs;
use std::path::Path;
use std::time::Instant;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use rust_xlsxwriter::Workbook;
use scraper::Html;

use booth_archiver::api_structs::items::ItemApiResponse;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::time_it;
use booth_archiver::zaphkiel::lazy_statics::*;
use booth_archiver::zaphkiel::utils::get_pb;
use booth_archiver::zaphkiel::xlsx::{save_book, write_all, write_headers};

fn main() {
    let start: Instant = Instant::now();

    let wishlist_pages: Vec<String> = time_it!(at once | "getting wishlist pages" => {
            let pages = get_all_wishlist_pages(&CLIENT);
            println!("number of pages = {}", pages.len());
            pages
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

    let all_items = time_it!(at once | "Extracting items" => all_item_numbers
        .par_iter()
        .progress_with(get_pb(all_item_numbers.len() as u64, "Extracting Items"))
        .map(|id| format!("https://booth.pm/en/items/{}.json", id))
        .filter_map(|url| CLIENT.get_one(url).ok())
        .filter_map(|item| serde_json::from_str::<ItemApiResponse>(&item).ok())
        .collect::<Vec<ItemApiResponse>>()
    );

    dbg!(all_items.len());

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
        let mut workbook= Workbook::new();
        let worksheet = workbook.add_worksheet();

        write_headers(worksheet).unwrap();

        write_all(worksheet, all_items);

        save_book(&mut workbook, "temp/book.xlsx");
    });

    time_it!("dumping" => CLIENT.dump_cache());
    println!("{}", CLIENT.get_cache_stats());
    println!("cache misses: {:}", CLIENT.get_cache_misses());
    println!("time taken: {:?}", start.elapsed());
}
