use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use indicatif::ParallelProgressIterator;
use lingual::Lang;
use rayon::prelude::*;
use rust_xlsxwriter::Workbook;

use booth_archiver::{debug, time_it, write_items_to_file};
use booth_archiver::api_structs::items::ItemApiResponse;
use booth_archiver::api_structs::wish_list_name_items::WishListNameItemsResponse;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::item_row::ItemRow;
use booth_archiver::models::translation;
use booth_archiver::models::web_client::WebScraper;
use booth_archiver::models::xlsx::{format_cols, save_book, write_all, write_headers};
use booth_archiver::zaphkiel::cache::Cache;
use booth_archiver::zaphkiel::utils::get_pb;

fn main() {
    let start: Instant = Instant::now();

    let cookie = fs::read_to_string("cookie.txt").unwrap();

    let client = WebScraper::new(cookie, true);

    let (wishlist_pages, _) = time_it!(at once | "getting wishlist pages" => {
            let (pages, changed) = get_all_wishlist_pages(&client);
            debug!(pages.len());
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
    debug!(all_item_numbers.len());

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
        debug!(client_get_one_errs.len());
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
        debug!(serde_json_errs.len());
    }

    debug!(all_items.len());

    write_items_to_file!(all_items);

    let item_rows = time_it!(at once | "converting items to item rows" => {
        all_items
            .par_iter()
            .progress_with(get_pb(all_items.len() as u64, "converting items to Item Rows"))
            .map(|item| item.to_owned().into())
            .collect::<Vec<ItemRow>>()
    });

    let mut path_to_cache = PathBuf::new();
    path_to_cache.push("cache");
    path_to_cache.push("translation.ron");

    let translation_cache = Arc::new(RwLock::new(Cache::new_with_path(path_to_cache)));

    let strings = time_it!("extracting strings from item rows" => {
        item_rows
            .iter()
            .flat_map(|item_row| {
                let markdown_strings = item_row
                    .markdown
                    .split('\n')
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>();
                let mut strings = vec![item_row.author_name.clone(), item_row.item_name.clone()];
                strings.extend(markdown_strings);
                strings
            })
            .collect::<Vec<_>>()
    });

    let translation_errors = Arc::new(Mutex::new(vec![]));

    let ctxs = Some(Arc::new(Mutex::new(vec![])));

    time_it!(at once | "caching translation for strings" => {
        strings
            .par_iter()
            .progress_with(get_pb(strings.len() as u64, "translating strings"))
            .for_each(|string| {
                match translation::translate(string, Lang::En, Some(translation_cache.clone()), ctxs.clone()) {
                    Ok(_) => {}
                    Err(err) => {
                        translation_errors
                            .clone()
                            .lock()
                            .unwrap()
                            .push((err, string.clone()));
                    }
                }
            });
    });

    if !translation_errors.lock().unwrap().is_empty() {
        let translation_errors = translation_errors.clone();
        let translation_errors = translation_errors.lock();
        let translation_errors = translation_errors.unwrap();
        let translation_errors = translation_errors
            .iter()
            .map(|(err, string)| (err, string.to_string()))
            .collect::<Vec<_>>();
        write_items_to_file!(translation_errors);
        debug!(translation_errors.len());
    }

    time_it!("dumping translation cache" => translation_cache.clone().write().unwrap().dump());

    let initial_translation_cache_stats = translation_cache.clone().read().unwrap().get_stats();

    write_items_to_file!(initial_translation_cache_stats);

    let translated_item_rows = time_it!(at once | "translating item rows" => {
        let len = item_rows.len();
        item_rows
            .into_par_iter()
            .progress_with(get_pb(len as u64, "translating Item Rows"))
            .map(|item_row| item_row.tl(translation_cache.clone(), ctxs.clone()).unwrap())
            .collect::<Vec<_>>()
    });

    time_it!("dumping translation cache" => translation_cache.clone().write().unwrap().dump());

    let final_translation_cache_stats = translation_cache.clone().read().unwrap().get_stats();

    write_items_to_file!(final_translation_cache_stats);

    time_it!(at once | "writing items to xlsx" => {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        write_headers(worksheet).unwrap();

        write_all(worksheet, translated_item_rows);

        format_cols(worksheet).unwrap();

        save_book(&mut workbook, "temp/book.xlsx");
    });

    time_it!("dumping cache" => cache.clone().write().unwrap().dump());

    let cache_stats = cache.clone().read().unwrap().get_stats();
    debug!(&cache_stats);

    write_items_to_file!(cache_stats);

    let cache_misses = cache.clone().read().unwrap().get_misses();
    if !cache_misses.is_empty() {
        debug!(cache_misses.len());
        debug!(cache_misses);
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

    let translation_ctxs = ctxs.unwrap().lock().unwrap().clone();

    write_items_to_file!(translation_ctxs);

    println!("whole program => {:#?}", start.elapsed());
}
