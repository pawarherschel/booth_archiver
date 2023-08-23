use scraper::{Html, Selector};

use crate::models::item_metadata::ItemInfo;
use crate::models::web_scrapper::WebScraper;
use crate::zaphkiel::static_strs::*;

/// Get the last page number of the wishlist.
fn get_last_page_number(client: &WebScraper) -> u32 {
    let document = client
        .get_one(BASE_BOOTH_WISHLIST_URL.to_string())
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

    let urls = (1..=last_page)
        .map(|page_number| format!("{}{}", BASE_BOOTH_WISHLIST_URL, page_number))
        .collect::<Vec<_>>();

    client
        .get_many(urls)
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .cloned()
        .collect()
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

/// Get all the item webpages on all wishlist pages.
///
/// # Arguments
///
/// * `client` - The client to use to get the webpages.
/// * `id` - The id of the item to get.
pub fn get_item(client: &WebScraper, id: u32) -> String {
    let url = format!("{}{}", BASE_BOOTH_ITEM_URL, id);
    client.get_one(url).unwrap_or_else(|e| {
        panic!(
            "failed to get item page for item number {} because of error: {}",
            id, e
        )
    })
}

/// Get all the item webpages from a list of item ids.
///
/// # Arguments
///
/// * `client` - The client to use to get the webpages.
/// * `ids` - The ids of the items to get.
pub fn get_items(client: &WebScraper, ids: Vec<u32>) -> Vec<String> {
    let urls = ids
        .iter()
        .map(|id| format!("{}{}", BASE_BOOTH_ITEM_URL, id))
        .collect::<Vec<_>>();
    client
        .get_many(urls)
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .cloned()
        .collect()
}

pub fn extract_image_urls_from_item_page(document: &Html) -> Option<Vec<String>> {
    let selector = Selector::parse(".market-item-detail-item-image").unwrap();
    let potential_images = document
        .select(&selector)
        .filter_map(|img| {
            img.value()
               .attrs()
               .find(|(k, _)| *k == "data-origin")
               .map(|(_, v)| v.to_string())
        })
        .collect::<Vec<_>>();

    if potential_images.is_empty() {
        println!("failed to get image from item page, either something happened or the page didnt have any");
        None
    } else {
        Some(potential_images)
    }
}

pub fn extract_image_urls_from_url(client: &WebScraper, url: String) -> Option<Vec<String>> {
    let doc = match client.get_one(url.clone()) {
        Ok(doc) => doc,
        Err(e) => {
            println!(
                "failed to get document from url: {}, because of error: {}",
                url, e
            );
            return None;
        }
    };
    let doc = Html::parse_document(&doc);
    let potential_images = extract_image_urls_from_item_page(&doc);

    match potential_images {
        None => {
            println!("failing url: {}", url);
            None
        }
        images => images,
    }
}

pub fn extract_item_data_from_item_page(document: &Html) -> Option<ItemInfo> {
    let shop_text_selector = Selector::parse("header.shop__text").unwrap();
    let shop_text_element = document
        .select(&shop_text_selector)
        .next()
        .expect("failed to get shop text");

    let shop_name_selector = Selector::parse("h2").unwrap();

    let shop_name = shop_text_element
        .select(&shop_name_selector)
        .next()
        .expect("failed to get shop name element")
        .inner_html();

    let images_selector = Selector::parse("img").unwrap();

    let images = shop_text_element
        .select(&images_selector)
        .collect::<Vec<_>>();

    let shop_name: String = images
        .iter()
        .map(|image| {
            image
                .value()
                .attrs()
                .find(|(k, v)| *k == "alt" && *v != "VRChat")
                .unwrap_or_default()
                .1
        })
        .collect();

    dbg!(shop_name);

    let anchor_selector = Selector::parse("a").unwrap();

    let shop_url: String = shop_text_element
        .select(&anchor_selector)
        .map(|a| a.value().attrs().find(|(k, _)| *k == "href").unwrap().1)
        .filter(|href| href.ends_with("booth.pm/"))
        .collect();

    dbg!(shop_url);

    // FIXME: nav hasnt loaded yet to extract categories
    // method 1
    // let nav_selector = Selector::parse("nav").unwrap();
    //
    // let nav_element = shop_text_element
    //     .select(&nav_selector)
    //     .next()
    //     .expect("failed to get nav element");
    //
    // let nav_anchors = nav_element.select(&anchor_selector).collect::<Vec<_>>();
    //
    // for anchor in nav_anchors {
    //     let href = anchor
    //         .value()
    //         .attrs()
    //         .find(|(k, _)| *k == "href")
    //         .unwrap()
    //         .1;
    //     let text = anchor.inner_html();
    //
    //     dbg!(href, text);
    // }
    //
    // method 2
    //
    // let category_selector = Selector::parse("item-card__category-anchor nav-reverse").unwrap();
    //
    // let category = shop_text_element
    //     .select(&category_selector)
    //     .next()
    //     .expect("failed to get category element")
    //     .inner_html();
    //
    // dbg!(category);

    todo!()
}

pub fn extract_item_data_from_url(client: &WebScraper, url: String) -> Option<ItemInfo> {
    let doc = match client.get_one(url.clone()) {
        Ok(doc) => doc,
        Err(e) => {
            println!(
                "failed to get document from url: {}, because of error: {}",
                url, e
            );
            return None;
        }
    };
    let doc = Html::parse_document(&doc);

    let potential_item_info = extract_item_data_from_item_page(&doc);

    match potential_item_info {
        None => {
            println!("failing url: {}", url);
            None
        }
        item_info => item_info,
    }
}
