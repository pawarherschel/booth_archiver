use fs::File;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use rust_xlsxwriter::Workbook;
use scraper::Html;

use booth_archiver::api_structs::items::ItemApiResponse;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::xlsx::{format_cols, save_book, write_all, write_headers};
use booth_archiver::time_it;
use booth_archiver::zaphkiel::cache::Cache;
use booth_archiver::zaphkiel::lazy_statics::*;
use booth_archiver::zaphkiel::utils::get_pb;

fn main() {
    let start: Instant = Instant::now();

    let (wishlist_pages, _last_page_changed) = time_it!(at once | "getting wishlist pages" => {
            let (pages, changed) = get_all_wishlist_pages(&CLIENT);
            println!("number of pages = {}", pages.len());
            (pages, changed)
        }
    );

    let all_item_numbers = time_it!(at once | "extracting item numbers from pages" => {
        wishlist_pages
        .par_iter()
        .progress_with(get_pb(wishlist_pages.len() as u64, "Extracting Item Numbers"))
        .flat_map(|o_page| {
            let page = Html::parse_document(o_page);
            get_all_item_numbers_on_page(&page)
        })
        .collect::<Vec<_>>()
    });

    println!("all_item_numbers.len(): {}", all_item_numbers.len());

    let mut path_to_cache = PathBuf::new();
    path_to_cache.push("cache");
    path_to_cache.push("all_items.ron");

    let cache = Arc::new(RwLock::new(Cache::new_with_path(path_to_cache)));

    let client_get_one_errs = Arc::new(Mutex::new(vec![]));
    let serde_json_errs = Arc::new(Mutex::new(vec![]));

    let all_items = time_it!(at once | "Extracting items" => all_item_numbers
        // .iter()
        .par_iter()
        .progress_with(get_pb(all_item_numbers.len() as u64, "Extracting Items"))
        .map(|id| format!("https://booth.pm/en/items/{}.json", id))
        .filter_map(|url| {
            match CLIENT.get_one(url, Some(cache.clone())) {
                Ok(item) => Some(item),
                Err(err) => {
                    client_get_one_errs.clone().lock().unwrap().push(err);
                    None
                }
            }
        })
        .filter_map(|item| {
            match serde_json::from_str::<ItemApiResponse>(&item) {
                Ok(item) => Some(item),
                Err(err) => {
                    serde_json_errs.clone().lock().unwrap().push(err);
                    None
                }
            }
        })
        .collect::<Vec<ItemApiResponse>>()
    );

    if !client_get_one_errs.lock().unwrap().is_empty() {
        println!("CLIENT.get_one errs: {:#?}", client_get_one_errs);
        time_it!(at once | "writing web client errors to ron file" => {
            let output_path = Path::new("temp/client_get_one_errs.ron");
            let output_array = client_get_one_errs.clone().lock().unwrap().iter().map(|err| err.to_string()).collect::<Vec<_>>();

            let mut file = File::create(output_path).unwrap();
            let client_get_one_errs_pretty = ron::ser::to_string_pretty(&output_array, Default::default()).unwrap();
            file.write_all(client_get_one_errs_pretty.as_bytes()).unwrap();
        });
    }
    println!(
        "client_get_one_errs.len(): {}",
        client_get_one_errs.lock().unwrap().len()
    );

    if !serde_json_errs.lock().unwrap().is_empty() {
        println!("serde_json errs: {:#?}", serde_json_errs);
        time_it!(at once | "writing serde json errors to ron file" => {
            let output_path = Path::new("temp/serde_json_errs.ron");
            let output_array = serde_json_errs.clone().lock().unwrap().iter().map(|err| err.to_string()).collect::<Vec<_>>();

            let mut file = File::create(output_path).unwrap();
            let serde_json_errs_pretty = ron::ser::to_string_pretty(&output_array, Default::default()).unwrap();
            file.write_all(serde_json_errs_pretty.as_bytes()).unwrap();
        });
    }
    println!(
        "serde_json_errs.len(): {}",
        serde_json_errs.lock().unwrap().len()
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
    println!(
        "number of cache misses; {}",
        cache.clone().read().unwrap().get_misses().len()
    );

    let stats = cache.clone().read().unwrap().get_stats();

    if stats.cache_hits + stats.cache_misses != stats.cache_size as u64 {
        println!(
            "cache hits ({}) + cache misses ({}) != cache size ({})",
            stats.cache_hits, stats.cache_misses, stats.cache_size
        );

        let misses = cache.clone().read().unwrap().get_misses();
        let hits = cache.clone().read().unwrap().get_hits();
        let all = cache
            .clone()
            .read()
            .unwrap()
            .cache
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        let missing = all
            .iter()
            .filter(|key| !misses.contains(key) && !hits.contains(key))
            .collect::<Vec<_>>();

        println!("missing: {:#?}", missing);
    }

    assert_eq!(Arc::strong_count(&cache), 1);

    println!("time taken: {:#?}", start.elapsed());
}
