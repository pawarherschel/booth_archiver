use std::error::Error;

use scraper::{Html, Selector};

use crate::models::web_scrapper::WebScraper;
use crate::zaphkiel::static_strs::*;

pub async fn get_last_page_number(client: &WebScraper) -> Result<u32, Box<dyn Error>> {
    let document = client.get_one(BASE_BOOTH_WISHLIST_URL.to_string()).await?;
    let selector = Selector::parse("a.nav-item.last-page")?;
    let last_page = document.select(&selector).last().expect(
        "failed to get last page, \
            are you sure the cookie is in the correct place and is valid?",
    );
    let href = last_page
        .value()
        .attr("href")
        .expect("the element didnt have an href");
    let page = href.split("page=").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .expect("failed to parse page number");
    Ok(page)
}

pub async fn get_all_wishlist_pages(client: &WebScraper) -> Result<Vec<Html>, Box<dyn Error>> {
    let last_page = get_last_page_number(client).await?;

    let urls = (1..=last_page)
        .map(|page_number| format!("{}{}", BASE_BOOTH_WISHLIST_URL, page_number))
        .collect::<Vec<_>>();

    let pages = client.get_many(urls).await?;

    Ok(pages)
}

pub async fn get_all_item_numbers_on_page(page: &Html) -> Result<Vec<u32>, Box<dyn Error>> {
    let selector =
        Selector::parse("body > div.page-wrap > main > div.manage-page-body > div > div > ul")?;

    let ul = page
        .select(&selector)
        .next()
        .expect("failed to get the list of items from the webpage");

    let selector = Selector::parse("li")?;

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

    Ok(items)
}

pub async fn get_item(client: &WebScraper, id: u32) -> Result<Html, Box<dyn Error>> {
    let url = format!("{}{}", BASE_BOOTH_ITEM_URL, id);
    let document = client.get_one(url).await?;
    Ok(document)
}

pub async fn get_items(client: &WebScraper, ids: Vec<u32>) -> Result<Vec<Html>, Box<dyn Error>> {
    let urls = ids
        .iter()
        .map(|id| format!("{}{}", BASE_BOOTH_ITEM_URL, id))
        .collect::<Vec<_>>();
    let documents = client.get_many(urls).await?;
    Ok(documents)
}
