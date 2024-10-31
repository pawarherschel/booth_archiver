// use crate::models::item_metadata::ItemMetadata;

use serde::{Deserialize, Serialize};

use crate::api_structs::items::ItemApiResponse;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ItemRow {
    pub item_name: String,
    pub item_name_translated: Option<String>,
    pub item_link: String,
    pub author_name: String,
    pub author_name_translated: Option<String>,
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
    pub markdown_translated: Option<String>,
}

#[allow(clippy::fallible_impl_from)]
impl From<ItemApiResponse> for ItemRow {
    fn from(value: ItemApiResponse) -> Self {
        let item_name = value.name;
        let item_name_translated = None;
        let item_link = value.url;
        let author_name = value.shop.name;
        let author_name_translated = None;
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
        let hearts = u32::try_from(value.wish_lists_count).unwrap();
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
        let markdown_translated = None;
        Self {
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
            markdown_translated,
        }
    }
}
