use scraper::{Html, Selector};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::models::web_scrapper::WebScraper;
use crate::zaphkiel::cache::Cache;
use crate::zaphkiel::static_strs::*;

/// Get the last page number of the wishlist.
fn get_last_page_number(client: &WebScraper) -> u32 {
    let document = client
        .get_one(BASE_BOOTH_WISHLIST_URL.to_string(), None)
        .unwrap_or_else(|e| panic!("failed to get wishlist page because of error: {}", e));
    let document = Html::parse_document(&document);
    let selector = Selector::parse("a.nav-item.last-page").unwrap();
    let last_page = document.select(&selector).last().expect(
        "failed to get last page, \
            are you sure the cookie is in the correct place and is valid",
    );
    let href = last_page
        .value()
        .attr("href")
        .expect("the element didnt have an href");
    let page = href.split("page=").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .expect("failed to parse page number");

    page
}

/// Get all the wishlist pages.
pub fn get_all_wishlist_pages(client: &WebScraper) -> Vec<String> {
    let last_page = get_last_page_number(client);

    let mut cache_path = PathBuf::new();
    cache_path.push("cache");
    cache_path.push("get_all_wishlist_pages.ron");

    let cache = Arc::new(RwLock::new(Cache::new_with_path(cache_path)));

    let urls = (1..=last_page)
        .map(|page_number| format!("{}{}", BASE_BOOTH_WISHLIST_URL, page_number))
        .collect::<Vec<_>>();

    let ret = client
        .get_many(urls, cache.clone(), "Getting all wishlist pages")
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .cloned()
        .collect();

    cache.clone().read().unwrap().dump();

    ret
}

/// Get all the item numbers on a wishlist page.
///
/// # Arguments
///
/// * `page` - The page to get the item numbers from.
pub fn get_all_item_numbers_on_page(page: &Html) -> Vec<u32> {
    let selector =
        Selector::parse("body > div.page-wrap > main > div.manage-page-body > div > div > ul")
            .unwrap();

    let ul = page
        .select(&selector)
        .next()
        .expect("failed to get the list of items from the webpage");

    let selector = Selector::parse("li").unwrap();

    let list = ul.select(&selector).collect::<Vec<_>>();

    let mut items = vec![];

    for item in list {
        item.value()
            .attrs()
            .filter(|&(key, _)| key == "data-product-id")
            .for_each(|(_, value)| {
                let item_number = value.parse::<u32>().unwrap_or_else(|e| {
                    panic!("failed to parse item number, {}, with error: {}", value, e)
                });
                items.push(item_number);
            });
    }

    items
}
