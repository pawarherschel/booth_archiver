use std::error::Error;

use scraper::{Html, Selector};

use crate::models::web_scrapper::WebScraper;
use crate::zaphkiel::static_strs::*;

pub async fn get_last_page_number(client: &WebScraper) -> Result<u32, Box<dyn Error>> {
    //<a
    //  class="nav-item last-page"
    //  href="/wish_lists?_gl=1%2A1k689pb%2A_ga%2AMTk4MzM0ODIyMi4xNjg0OTU4ODIw%2A_ga_RWT2QKJLDC%2AMTY5MDU0OTA5Ny4yMS4wLjE2OTA1NDkwOTcuMC4wLjA.&amp;page=43">
    //  <i
    //      class="icon-angle-double-right no-margin s-1x">
    //  </i>
    // </a>

    let document = client.get_one(BASE_BOOTH_WISHLIST_URL.to_string()).await?;
    let selector = Selector::parse("a.nav-item.last-page").unwrap();
    let last_page = document.select(&selector).last().unwrap();
    let href = last_page.value().attr("href").unwrap();
    let page = href.split("page=").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
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
        Selector::parse("body > div.page-wrap > main > div.manage-page-body > div > div > ul")
            .unwrap();

    let ul = page.select(&selector).next().unwrap();

    let selector = Selector::parse("li").unwrap();

    let list = ul.select(&selector).collect::<Vec<_>>();

    let mut items = vec![];

    for item in list {
        item.value()
            .attrs()
            .filter(|&(key, _)| key == "data-product-id")
            .for_each(|(_, value)| {
                let item_number = value.parse::<u32>().unwrap();
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
