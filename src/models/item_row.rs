use crate::models::item_metadata::ItemMetadata;

// Item Name	Item Name Translated	Item Link	Author Name	Author Name Translated	Author Link
// Category	VRChat Badge	Adult Badge	Price	Currency	Wish List Count
// Images Number	Image URLs	Downloads Number
// Downloads Dict	Download Links	Download Names	Download Variations
// Download Formats	Download Sizes	Download Units	Description Markdown

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
    pub price: f64,
    pub currency: String,
    pub hearts: u32,
    pub image_urls: Vec<String>,
    pub downloads_links: Vec<String>,
    pub markdown: String,
}

impl From<ItemMetadata> for ItemRow {
    fn from(value: ItemMetadata) -> Self {
        let item_name = value.item.name.name.name;
        let item_name_translated = value.item.name.name.name_translated;
        let item_link = value.item.name.url;

        let author_name = value.author.name.name;
        let author_name_translated = value.author.name.name_translated;
        let author_link = value.author.url;

        let primary_category = value.category.category.name.name;
        let secondary_category = value.category.subcategory.name.name;

        let vrchat = value.badges.vrchat;
        let adult = value.badges.adult;

        let price = value.price.number;
        let currency = value.price.unit;

        let hearts = value.hearts;

        let image_urls = value.images.iter().map(|image| image.url.clone()).collect();

        let downloads_links = value
            .downloads
            .iter()
            .map(|download| download.name.url.clone())
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
            price,
            currency,
            hearts,
            image_urls,
            downloads_links,
            markdown,
        }
    }
}
