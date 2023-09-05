// use crate::models::item_metadata::ItemMetadata;

use crate::api_structs::items::ItemApiResponse;

#[derive(Clone, Default, Debug)]
pub struct ItemRow {
    pub item_name: String,
    pub item_name_translated: String,
    pub item_link: String,
    pub author_name: String,
    pub author_name_translated: String,
    pub author_link: String,
    pub primary_category: String,
    pub secondary_category: String,
    pub vrchat: bool,
    pub adult: bool,
    pub tags: Vec<String>,
    pub price: f64,
    pub currency: String,
    pub hearts: u32,
    pub image_urls: Vec<String>,
    pub download_links: Vec<String>,
    pub markdown: String,
}

impl From<ItemApiResponse> for ItemRow {
    fn from(value: ItemApiResponse) -> Self {
        let item_name = value.name;
        let item_name_translated = item_name.clone();
        let item_link = value.url;
        let author_name = value.shop.name;
        let author_name_translated = author_name.clone();
        let author_link = value.shop.url;
        let primary_category = value.category.parent.name;
        let secondary_category = value.category.name;
        let vrchat = value
            .tags
            .iter()
            .any(|tag| tag.name.to_lowercase() == "vrchat");
        let adult = value.is_adult;
        let tags = value.tags.iter().map(|tag| tag.name.clone()).collect();
        let price_tuple: (&str, &str) = value.price.split_once(' ').unwrap();
        let price = price_tuple.0.replace(',', "").parse().unwrap();
        let currency = price_tuple.1.to_owned();
        let hearts = value.wish_lists_count as u32;
        let image_urls = value
            .images
            .iter()
            .map(|img| img.original.clone())
            .collect();
        let download_links = value
            .variations
            .iter()
            .flat_map(|variation| {
                variation.downloadable.iter().flat_map(|downloadable| {
                    downloadable.no_musics.iter().map(|item| item.url.clone())
                })
            })
            .collect();
        let markdown = value.description;
        ItemRow {
            item_name,
            item_name_translated,
            item_link,
            author_name,
            author_name_translated,
            author_link,
            primary_category,
            secondary_category,
            vrchat,
            adult,
            tags,
            price,
            currency,
            hearts,
            image_urls,
            download_links,
            markdown,
        }
    }
}

// impl From<ItemMetadata> for ItemRow {
//     fn from(value: ItemMetadata) -> Self {
//         let item_name = value.item.name.name.name;
//         let item_name_translated = value.item.name.name.name_translated;
//         let item_link = value.item.name.url;
//
//         let author_name = value.author.name.name;
//         let author_name_translated = value.author.name.name_translated;
//         let author_link = value.author.url;
//
//         let primary_category = value.category.category.name.name;
//         let secondary_category = value.category.subcategory.name.name;
//
//         let vrchat = value.badges.vrchat;
//         let adult = value.badges.adult;
//
//         let price = value.price.number;
//         let currency = value.price.unit;
//
//         let hearts = value.hearts;
//
//         let image_urls = value.images.iter().map(|image| image.url.clone()).collect();
//
//         let downloads_links = value
//             .downloads
//             .iter()
//             .map(|download| download.name.url.clone())
//             .collect();
//
//         let markdown = value.description;
//
//         ItemRow {
//             item_name,
//             item_name_translated,
//             item_link,
//             author_name,
//             author_name_translated,
//             author_link,
//             primary_category,
//             secondary_category,
//             vrchat,
//             adult,
//             price,
//             currency,
//             hearts,
//             image_urls,
//             downloads_links,
//             markdown,
//         }
//     }
// }
