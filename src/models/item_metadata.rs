use serde::{Deserialize, Serialize};

use crate::api_structs::items::ItemApiResponse;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemMetadata {
    pub item: ItemInfo,
    pub author: NameWithUrl,
    pub hearts: u32,
    pub price: NumberWithUnit,
    pub category: CategoryInfo,
    pub images: Vec<NameWithUrl>,
    pub downloads: Vec<DownloadInfo>,
    pub badges: Badges,
    pub description: String,
}

impl ItemMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<ItemApiResponse> for ItemMetadata {
    fn from(value: ItemApiResponse) -> Self {
        let item = value.name.trim();
        let author = value.shop;
        let hearts = value.wish_lists_count;
        let price = value.price;
        let category = value.category;
        let images = value.images;
        let variations = value.variations;

        let item = {
            let id = u32::try_from(value.id).unwrap_or_else(|e| {
                panic!(
                    "unable to convert id ({}) into u32 because of error {}",
                    value.id, e
                )
            });
            let name = item.to_owned().into();
            let url = value.url;
            ItemInfo {
                name: NameWithUrl { name, url },
                id,
            }
        };
        let author = {
            let name = author.name.into();
            let url = author.url;
            NameWithUrl { name, url }
        };
        let hearts = u32::try_from(hearts).unwrap_or_else(|e| {
            panic!(
                "unable to convert hearts ({}) into u32 because of error {}",
                hearts, e
            )
        });
        let price = {
            let temp = price.split(' ').collect::<Vec<_>>();
            if temp.len() != 2 {
                panic!("price ({}) is not in the correct format", price);
            }
            let number = temp[0].replace(',', "").parse::<f64>().unwrap_or_else(|e| {
                panic!(
                    "unable to convert price ({}) into f32 because of error {}",
                    temp[0], e
                )
            });
            let unit = temp[1].to_owned();
            NumberWithUnit { number, unit }
        };
        let category = {
            let subcategory_url = category.url;
            let subcategory = category.name;
            let category_url = category.parent.url;
            let category = category.parent.name;

            let category = NameWithUrl {
                name: category.into(),
                url: category_url,
            };
            let subcategory = NameWithUrl {
                name: subcategory.into(),
                url: subcategory_url,
            };
            CategoryInfo {
                category,
                subcategory,
            }
        };
        let images = {
            images
                .into_iter()
                .map(|image| {
                    let name = image.caption.unwrap_or_default().to_string();
                    let name = name.into();
                    let url = image.original;
                    NameWithUrl { name, url }
                })
                .collect::<Vec<_>>()
        };

        let downloads = variations
            .iter()
            .flat_map(|variation| {
                variation.downloadable.iter().map(|downloadable| {
                    downloadable
                        .no_musics
                        .iter()
                        .map(|download| download.url.clone())
                        .collect::<Vec<_>>()
                })
            })
            .flatten()
            .map(|url| DownloadInfo {
                name: NameWithUrl {
                    name: NameWithTranslation::default(),
                    url: url.clone(),
                },
                price: NumberWithUnit::default(),
                variation: None,
                format: None,
                size: None,
            })
            .collect::<Vec<_>>();

        let badges = {
            let adult = value.is_adult;

            let vrchat = value.tags.iter().any(|tag| tag.name == "VRChat");

            Badges { vrchat, adult }
        };

        let description = value.description;

        ItemMetadata {
            item,
            author,
            hearts,
            price,
            category,
            images,
            downloads,
            badges,
            description,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemInfo {
    pub name: NameWithUrl,
    pub id: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub category: NameWithUrl,
    pub subcategory: NameWithUrl,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    pub name: NameWithUrl,
    pub price: NumberWithUnit,
    pub variation: Option<NameWithTranslation>,
    pub format: Option<String>,
    pub size: Option<NumberWithUnit>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameWithTranslation {
    pub name: String,
    pub name_translated: String,
}

impl From<String> for NameWithTranslation {
    fn from(val: String) -> Self {
        NameWithTranslation {
            name: val.clone(),
            name_translated: val,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameWithUrl {
    pub name: NameWithTranslation,
    pub url: String,
}

impl NameWithUrl {
    pub fn new(name: NameWithTranslation, url: String) -> Self {
        Self { name, url }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Badges {
    pub vrchat: bool,
    pub adult: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NumberWithUnit {
    pub number: f64,
    pub unit: String,
}

impl NumberWithUnit {
    pub fn new(number: f64, unit: String) -> Self {
        Self { number, unit }
    }
}
