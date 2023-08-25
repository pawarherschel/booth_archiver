use fs::File;
use std::fs;
use std::path::Path;
use std::time::Instant;

use rayon::prelude::*;
use rust_xlsxwriter::{Workbook, XlsxError};
use scraper::Html;

use booth_archiver::api_structs::items::ItemApiResponse;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::item_metadata::ItemMetadata;
use booth_archiver::time_it;
use booth_archiver::zaphkiel::lazy_statics::*;
use booth_archiver::zaphkiel::utils::{
    check_if_the_unneeded_files_are_generated_and_panic_if_they_do, unneeded_values,
};
use booth_archiver::zaphkiel::xlsx::{write_headers, write_row};

fn main() {
    let start: Instant = Instant::now();

    let wishlist_pages: Vec<String> = time_it!(at once | "getting wishlist pages" => {
            let pages = get_all_wishlist_pages(&CLIENT);
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
        .par_iter()
        .map(|id| format!("https://booth.pm/en/items/{}.json", id))
        .collect::<Vec<_>>();

    let all_item_json = time_it!(at once | "getting all items json" => {
        CLIENT
            .get_many(all_items_json_url)
            .par_iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
    });

    let mut errors = Vec::new();
    let mut error_json = Vec::new();

    let all_items = all_item_json
        .iter()
        .filter_map(|s| match serde_json::from_str::<ItemApiResponse>(s) {
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
        time_it!(at once | "writing errors to file" => {
            let mut error_file = File::create("errors.json").unwrap();
            let mut error_json = error_json.join(",");
            error_json.insert(0, '[');
            error_json.push(']');
            error_file.write_all(error_json.as_bytes()).unwrap();
        });
    }

    unneeded_values(&all_items);
    check_if_the_unneeded_files_are_generated_and_panic_if_they_do();

    let all_items = all_items
        .par_iter()
        .map(|x| ItemMetadata::from(x.clone()))
        .collect::<Vec<ItemMetadata>>();

    time_it!(at once | "writing items to file" => {
        let output_path = Path::new("temp/items.ron");

        let mut file = File::create(output_path).unwrap();
        let all_items_pretty = ron::ser::to_string_pretty(&all_items, Default::default()).unwrap();
        file.write_all(all_items_pretty.as_bytes()).unwrap();
    });

    time_it!(at once | "writing items to xlsx" => {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        write_headers(worksheet).unwrap();

        all_items
            .iter()
            .map(|item| item.to_owned().into())
            .enumerate()
            .for_each(|(idx, item)| write_row(&item, worksheet, idx as u32 + 1).unwrap());

        match workbook.save("temp/book.xlsx") {
            Ok(_) => println!("saved"),
            Err(e) => match e {
                XlsxError::IoError(e) => println!("io error: {}\n\
                Did you check if the file is already open in excel?", e),
                _ => println!("error: {}", e),
            }
        };
    });

    time_it!("dumping" => CLIENT.dump_cache());
    println!("{}", CLIENT.get_cache_stats());
    println!("cache misses: {:}", CLIENT.get_cache_misses());
    println!("time taken: {:?}", start.elapsed());
}
