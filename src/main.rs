use fs::File;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use rust_xlsxwriter::Workbook;

use booth_archiver::api_structs::items::ItemApiResponse;
use booth_archiver::api_structs::wish_list_name_items::WishListNameItemsResponse;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::web_scrapper::WebScraper;
use booth_archiver::models::xlsx::{format_cols, save_book, write_all, write_headers};
use booth_archiver::zaphkiel::cache::Cache;
use booth_archiver::zaphkiel::pub_consts::DBG;
use booth_archiver::zaphkiel::utils::get_pb;
use booth_archiver::{time_it, write_items_to_file};

pub const COOKIE: &str = include_str!("../cookie.txt");

fn main() {
    let start: Instant = Instant::now();

    let client = WebScraper::new(COOKIE.to_string(), true);

    let (wishlist_pages, _last_page_changed) = time_it!(at once | "getting wishlist pages" => {
            let (pages, changed) = get_all_wishlist_pages(&client);
            if DBG {
                dbg!(pages.len());
            }
            (pages, changed)
        }
    );

    let all_item_numbers = time_it!(at once | "extracting item numbers from pages" => {
        wishlist_pages
        .par_iter()
        .progress_with(get_pb(wishlist_pages.len() as u64, "extracting Item Numbers"))
        .flat_map(|o_page| {
            let page = serde_json::from_str::<WishListNameItemsResponse>(o_page).unwrap();
            get_all_item_numbers_on_page(&page)
        })
        .collect::<Vec<_>>()
    });
    if DBG {
        dbg!(all_item_numbers.len());
    }

    let mut path_to_cache = PathBuf::new();
    path_to_cache.push("cache");
    path_to_cache.push("all_items.ron");

    let cache = Arc::new(RwLock::new(Cache::new_with_path(path_to_cache)));

    let client_get_one_errs = Arc::new(Mutex::new(vec![]));
    let serde_json_errs = Arc::new(Mutex::new(vec![]));

    let all_items = time_it!(at once | "extracting items" => all_item_numbers
        .par_iter()
        .progress_with(get_pb(all_item_numbers.len() as u64, "extracting Items"))
        .map(|id| format!("https://booth.pm/en/items/{}.json", id))
        .filter_map(|url| {
            match client.get_one(url, Some(cache.clone())) {
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
        let client_get_one_errs = client_get_one_errs
            .clone()
            .lock()
            .unwrap()
            .iter()
            .map(|err| err.to_string())
            .collect::<Vec<_>>();
        write_items_to_file!(client_get_one_errs);
        if DBG {
            dbg!(client_get_one_errs.len());
        }
    }

    if !serde_json_errs.lock().unwrap().is_empty() {
        let serde_json_errs = serde_json_errs
            .clone()
            .lock()
            .unwrap()
            .iter()
            .map(|err| err.to_string())
            .collect::<Vec<_>>();
        write_items_to_file!(serde_json_errs);
        if DBG {
            dbg!(serde_json_errs.len());
        }
    }

    if DBG {
        dbg!(all_items.len());
    }

    write_items_to_file!(all_items);

    time_it!(at once | "writing items to xlsx" => {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        write_headers(worksheet).unwrap();

        write_all(worksheet, all_items);

        format_cols(worksheet).unwrap();

        save_book(&mut workbook, "temp/book.xlsx");
    });

    time_it!("dumping cache" => cache.clone().write().unwrap().dump());

    let cache_stats = cache.clone().read().unwrap().get_stats();
    if DBG {
        dbg!(&cache_stats);
    }

    write_items_to_file!(cache_stats);

    let cache_misses = cache.clone().read().unwrap().get_misses();
    if !cache_misses.is_empty() && DBG {
        dbg!(cache_misses.len());
        dbg!(cache_misses);
    }

    if cache_stats.cache_hits + cache_stats.cache_misses != cache_stats.cache_size as u64 {
        write_items_to_file!(&cache_stats);

        println!(
            "cache hits ({}) + cache misses ({}) != cache size ({})",
            cache_stats.cache_hits, cache_stats.cache_misses, cache_stats.cache_size,
        );

        let misses = cache.clone().read().unwrap().get_misses();
        let hits = cache.clone().read().unwrap().get_hits();
        let all = cache.clone().read().unwrap().keys().collect::<Vec<_>>();

        let missing = all
            .iter()
            .filter(|key| !misses.contains(key) && !hits.contains(key))
            .collect::<Vec<_>>();

        println!("missing: {:#?}", missing);
        write_items_to_file!(missing);
    }

    assert_eq!(Arc::strong_count(&cache), 1);

    println!("whole program => {:#?}", start.elapsed());
}
