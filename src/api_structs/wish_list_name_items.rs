use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct WishListNameItemsResponse {
    pub items: Vec<Item>,
    pub pagination: Pagination,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub category: Category,
    pub event: Option<Event>,
    pub id: i64,
    pub is_adult: bool,
    pub is_end_of_sale: bool,
    pub is_placeholder: bool,
    pub is_sold_out: bool,
    pub is_vrchat: bool,
    pub minimum_stock: Option<i64>,
    pub music: Option<Music>,
    pub name: String,
    pub price: String,
    pub shop: Shop,
    pub thumbnail_image_urls: Vec<String>,
    pub url: String,
    pub wish_list_url: String,
    pub tracking_data: TrackingData,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    pub name: Name,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Name {
    pub en: String,
    pub ja: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Music {
    pub full_url: String,
    pub short_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shop {
    pub thumbnail_url: String,
    pub name: String,
    pub url: String,
    pub verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackingData {
    pub product_id: i64,
    pub product_price: i64,
    pub product_brand: String,
    pub product_category: i64,
    pub tracking: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    pub current_page: i64,
    pub prev_page: Option<i64>,
    pub next_page: Option<i64>,
    pub limit_value: i64,
    pub total_pages: i64,
    pub total_count: i64,
}
