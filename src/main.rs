use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use scraper::Html;

use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::web_scrapper::WebScraper;
use booth_archiver::temp::testing_urls::TESTING_URLS;
use booth_archiver::time_it;
use booth_archiver::zaphkiel::lazy_statics::*;
use booth_archiver::zaphkiel::static_strs::BASE_BOOTH_ITEM_URL;
use booth_archiver::zaphkiel::utils::get_pb;

/// TODO: make it so the cache location is separate for every class of url, eg: wishlist pages, item pages, images, etc.
/// TODO: cache the images
/// TODO: use the fucking api dumbass: format!("https://booth.pm/en/items/{}.json", item_id)
fn main() {
    let start = std::time::Instant::now();
    // test area

    dbg!(CLIENT.get_one(format!("https://booth.pm/en/items/{}.json", 1903612)))
        .expect("TODO: panic message");

    panic!();

    let page = Html::parse_document(&CLIENT.get_one(TESTING_URLS[0].to_string()).unwrap());
    let info = extract_item_data_from_item_page(&page).unwrap();

    // test area over
    let client = time_it!(at once | "creating client" =>
        WebScraper::new(COOKIE.to_string(), true)
    );

    let wishlist_pages = time_it!(at once | "getting wishlist pages" =>{
            let pages = get_all_wishlist_pages(&client);
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

    let all_item_pages_urls = all_item_numbers
        .iter()
        .map(|item_number| format!("{}{}", BASE_BOOTH_ITEM_URL, item_number))
        .collect::<Vec<_>>();

    println!("number of items = {}", all_item_numbers.len());

    let all_items = time_it!(at once | "downloading all item pages" =>
        get_items(&client, all_item_numbers)
    );
    time_it!("dumping" => client.dump_cache());
    println!("number of items: {:}", all_items.len());

    let images = time_it!(at once | "extracting images from all items" =>
        all_item_pages_urls
            .par_iter()
            .progress_with(get_pb(all_item_pages_urls.len() as u64))
            .map(|item| extract_image_urls_from_url(&client, item.clone()))
            .flatten()
            .flatten()
            .collect::<Vec<_>>()
    );

    println!("number of images: {:}", images.len());

    println!("{}", client.get_cache_stats());
    println!("cache misses: {:}", client.get_cache_misses());
    println!("time taken: {:?}", start.elapsed());
}
