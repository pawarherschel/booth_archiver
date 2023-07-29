use lazy_static::lazy_static;
use path_absolutize::Absolutize;
use rayon::prelude::*;
use scraper::Html;

use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::config::Config;
use booth_archiver::models::web_scrapper::WebScraper;
use booth_archiver::time_it;
use booth_archiver::zaphkiel::static_strs::BASE_BOOTH_ITEM_URL;

lazy_static! {
    /// The config for the program.
    pub static ref CONFIG: Config = time_it!("loading config" => Config::get());

    /// The cookie for the program.
    pub static ref COOKIE: String = {
        std::fs::read_to_string(
            CONFIG
                .cookie_file
                .as_ref()
                .expect("failed to build Path from PathBuf"),
        )
        .unwrap_or_else(|e| {
            panic!(
                "expecting cookie to be in {}, because of error: {}",
                CONFIG
                    .cookie_file
                    .as_ref()
                    .expect(
                        "failed to build Path from PathBuf, \
                        imagine panicking inside a panic lmao"
                    )
                    .absolutize()
                    .expect(
                        "failed to absolutize path from PathBuf, \
                        imagine panicking inside a panic lmao"
                    )
                    .to_str()
                    .expect("failed to convert path to str, imagine panicking inside a panic lmao"),
                e
            )
        })
    };
}

fn main() {
    let start = std::time::Instant::now();
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
