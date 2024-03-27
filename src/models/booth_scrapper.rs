use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use ron::ser::PrettyConfig;

use crate::api_structs::wish_list_name_items::WishListNameItemsResponse;
use crate::models::web_client::WebScraper;
use crate::zaphkiel::cache::Cache;

/// Get the last page number of the wishlist.
fn get_last_page_number(client: &WebScraper) -> u32 {
    let document = client
        .get_one(
            "https://accounts.booth.pm/wish_list_name_items.json?page=1".to_string(),
            None,
        )
        .unwrap_or_else(|e| panic!("failed to get wishlist page because of error: {}", e));
    let document =
        serde_json::from_str::<WishListNameItemsResponse>(&document).unwrap_or_else(|e| {
            panic!(
                "failed to parse wishlist page as json because of error: {}\n\
                document: {}",
                e, document
            )
        });
    let last_page = document.pagination.total_pages;
    let last_page = u32::try_from(last_page).unwrap();

    ron::ser::to_writer_pretty(
        File::create("cache/last_page.ron").unwrap(),
        &last_page,
        PrettyConfig::default(),
    )
    .unwrap();

    last_page
}

/// Get all the wishlist pages.
#[must_use]
pub fn get_all_wishlist_pages(client: &WebScraper) -> (Vec<String>, bool) {
    let prev_last_page = if fs::metadata("cache/last_page.ron").is_ok() {
        ron::de::from_reader(File::open("cache/last_page.ron").unwrap()).unwrap()
    } else {
        0
    };

    let last_page = get_last_page_number(client);

    let last_page_changed = prev_last_page != last_page;

    let mut cache_path = PathBuf::new();
    cache_path.push("cache");
    cache_path.push("get_all_wishlist_pages.ron");

    let cache = Arc::new(RwLock::new(Cache::new_with_path(cache_path)));

    if last_page_changed {
        println!("last page changed, clearing cache");
        cache.write().unwrap().clear();
    }

    let urls = (1..=last_page)
        .map(|page_number| {
            format!(
                "https://accounts.booth.pm/wish_list_name_items.json?page={}",
                page_number
            )
        })
        .collect::<Vec<_>>();

    let ret = client
        .get_many(urls, cache.clone(), "Getting all wishlist pages")
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .cloned()
        .collect();

    cache.read().unwrap().dump();

    assert_eq!(Arc::strong_count(&cache), 1);

    (ret, last_page_changed)
}

/// Get all the item numbers on a wishlist page.
///
/// # Arguments
///
/// * `page` - The page to get the item numbers from.
#[inline]
#[must_use]
pub fn get_all_item_numbers_on_page(page: &WishListNameItemsResponse) -> Vec<u32> {
    let items = page
        .items
        .iter()
        .map(|item| u32::try_from(item.tracking_data.product_id).unwrap())
        .collect::<Vec<_>>();

    items
}
