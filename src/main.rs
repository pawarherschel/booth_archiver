use fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;

use rayon::prelude::*;
use rust_xlsxwriter::Workbook;
use scraper::Html;

use booth_archiver::api_structs::items::ItemApiResponse;
use booth_archiver::models::booth_scrapper::*;
use booth_archiver::models::item_metadata::ItemMetadata;
use booth_archiver::models::web_scrapper::WebScraper;
use booth_archiver::time_it;
use booth_archiver::zaphkiel::lazy_statics::*;
use booth_archiver::zaphkiel::utils::{
    check_if_the_unneeded_files_are_generated_and_panic_if_they_do, unneeded_values,
};
use booth_archiver::zaphkiel::xlsx::write_row;

/// TODO: make it so the cache location is separate for every class of url, eg: wishlist pages, item pages, images, etc.
/// TODO: cache the images
/// TODO: use the fucking api dumbass: format!("https://booth.pm/en/items/{}.json", item_id)
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

    let all_items_json_url = all_item_numbers
        .iter()
        .map(|id| format!("https://booth.pm/en/items/{}.json", id))
        .collect::<Vec<_>>();

    let all_item_json = time_it!(at once | "getting all items json" => {
        client.get_many(all_items_json_url).iter().flatten().map(|s|s.to_owned()).collect::<Vec<_>>()
    });

    let mut errors = Vec::new();
    let mut error_json = Vec::new();

    let all_items = all_item_json
        .iter()
        .filter_map(|s| match serde_json::from_str::<ItemApiResponse>(s) {
            Ok(root) => Some(root),
            Err(e) => {
                errors.push(e.to_string());
                error_json.push(s.to_owned());
                None
            }
        })
        .collect::<Vec<_>>();

    println!("number of successes: {}", all_items.len());
    println!("number of errors: {}", errors.len());

    if !errors.is_empty() {
        let mut error_file = File::create("errors.json").unwrap();
        let mut error_json = error_json.join(",");
        error_json.insert(0, '[');
        error_json.push(']');
        error_file.write_all(error_json.as_bytes()).unwrap();
    }

    unneeded_values(&all_items);
    check_if_the_unneeded_files_are_generated_and_panic_if_they_do();

    let all_items: Vec<ItemMetadata> = all_items
        .iter()
        .map(|x| ItemMetadata::from(x.clone()))
        .collect();

    let output_path = Path::new("temp/items.ron");

    let mut file = File::create(output_path).unwrap();
    let all_items_pretty = ron::ser::to_string_pretty(&all_items, Default::default()).unwrap();
    file.write_all(all_items_pretty.as_bytes()).unwrap();

    let mut workbook = time_it!("Creating a new Workbook" => Workbook::new());
    let worksheet = time_it!("Adding worksheet" => workbook.add_worksheet());
    // Item Name	Item Name Translated	Item Link	Author Name	Author Name Translated	Author Link
    // Category	VRChat Badge	Adult Badge	Price	Currency	Wish List Count
    // Images Number	Image URLs	Downloads Number
    // Downloads Dict	Download Links	Download Names	Download Variations
    // Download Formats	Download Sizes	Download Units	Description Markdown
    time_it!("writing col names" => {
        worksheet.write(0, 0, "Item Name").unwrap();
        worksheet.write(0, 1, "Item Name Translated").unwrap();
        worksheet.write(0, 2, "Item Link").unwrap();
        worksheet.write(0, 3, "Author Name").unwrap();
        worksheet.write(0, 4, "Author Name Translated").unwrap();
        worksheet.write(0, 5, "Author Link").unwrap();
        worksheet.write(0, 6, "Category").unwrap();
        worksheet.write(0, 7, "VRChat Badge").unwrap();
        worksheet.write(0, 8, "Adult Badge").unwrap();
        worksheet.write(0, 9, "Price").unwrap();
        worksheet.write(0, 10, "Currency").unwrap();
        worksheet.write(0, 11, "Wishlist Count").unwrap();
        worksheet.write(0, 12, "Images Number").unwrap();
        worksheet.write(0, 13, "Images URLs").unwrap();
        worksheet.write(0, 14, "Downloads Numbers").unwrap();
        worksheet.write(0, 15, "Downloads Dict").unwrap();
        worksheet.write(0, 16, "Downloads Links").unwrap();
        worksheet.write(0, 17, "Downloads Names").unwrap();
        worksheet.write(0, 18, "Downloads Variations").unwrap();
        worksheet.write(0, 19, "Downloads Formats").unwrap();
        worksheet.write(0, 20, "Downloads Sizes").unwrap();
        worksheet.write(0, 21, "Downloads Units").unwrap();
        worksheet.write(0, 22, "Downloads Markdown").unwrap();
    });

    time_it!("Writing Data" => {
        for (idx, item) in all_items.iter().enumerate() {
            write_row(item, worksheet, idx as u32 + 1).unwrap();
        }
    });

    time_it!("saving workbook" => workbook.save("temp/book.xlsx").unwrap());

    time_it!("dumping" => client.dump_cache());
    println!("{}", client.get_cache_stats());
    println!("cache misses: {:}", client.get_cache_misses());
    println!("time taken: {:?}", start.elapsed());
}
