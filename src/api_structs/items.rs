use serde::{Deserialize, Serialize};
use serde_json::Value;

// TODO: when the files are generated, remove the unneeded serde_json::Value slowly and convert it to actual struct
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ItemApiResponse {
    pub description: String,
    pub factory_description: Option<Value>,
    pub id: i64,
    pub is_adult: bool,
    pub is_buyee_possible: bool,
    pub is_end_of_sale: bool,
    pub is_placeholder: bool,
    pub is_sold_out: bool,
    pub name: String,
    pub price: String,
    pub purchase_limit: Option<i64>,
    pub shipping_info: String,
    pub small_stock: Option<i64>,
    pub url: String,
    pub wish_list_url: String,
    pub wish_lists_count: i64,
    pub wished: bool,
    pub buyee_variations: Vec<BuyeeVariation>,
    pub category: Category,
    pub embeds: Vec<String>,
    pub images: Vec<Image>,
    pub order: Option<Value>,
    pub share: Share,
    pub shop: Shop,
    pub sound: Option<Sound>,
    pub tags: Vec<Tag>,
    pub tag_banners: Vec<TagBanner>,
    pub tag_combination: Option<TagCombination>,
    pub tracks: Option<Value>,
    pub variations: Vec<Variation>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct BuyeeVariation {
    pub buyee_html: String,
    pub downloadable: Option<Value>,
    pub factory_image_url: Option<Value>,
    pub has_download_code: bool,
    pub id: i64,
    pub is_anshin_booth_pack: bool,
    pub is_empty_allocatable_stock_with_preorder: bool,
    pub is_empty_stock: bool,
    pub is_factory_item: bool,
    pub is_mailbin: bool,
    pub is_waiting_on_arrival: bool,
    pub name: Option<String>,
    pub order_url: Option<Value>,
    pub price: i64,
    pub small_stock: Option<i64>,
    pub status: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent: Parent,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parent {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    // never used so far lol
    pub caption: Option<Value>,
    pub original: String,
    pub resized: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Share {
    pub hashtags: Vec<String>,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shop {
    pub name: String,
    pub subdomain: String,
    pub thumbnail_url: String,
    pub url: String,
    pub verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sound {
    pub full_url: String,
    pub short_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagBanner {
    pub image_url: Option<String>,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagCombination {
    pub category: String,
    pub tag: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Variation {
    pub buyee_html: Option<String>,
    pub downloadable: Option<Downloadable>,
    pub factory_image_url: Option<Value>,
    pub has_download_code: bool,
    pub id: i64,
    pub is_anshin_booth_pack: bool,
    pub is_empty_allocatable_stock_with_preorder: bool,
    pub is_empty_stock: bool,
    pub is_factory_item: bool,
    pub is_mailbin: bool,
    pub is_waiting_on_arrival: bool,
    pub name: Option<String>,
    pub order_url: Option<Value>,
    pub price: f64,
    pub small_stock: Option<i64>,
    pub status: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Downloadable {
    pub musics: Option<Vec<Value>>,
    pub no_musics: Vec<NoMusic>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoMusic {
    pub file_name: String,
    pub file_extension: String,
    pub file_size: String,
    pub name: String,
    pub url: String,
}
