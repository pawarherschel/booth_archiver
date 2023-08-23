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
            .map(|variation| {
                let name = variation.name.clone().unwrap_or_default().into();
                let name = NameWithUrl {
                    name,
                    url: variation.order_url.clone().unwrap_or_default().to_string(),
                };
                let price = variation.price;
                let price = NumberWithUnit {
                    number: price.try_into().unwrap_or_else(|e| {
                        panic!(
                            "Unable to convert price from {} to f64 because of {}",
                            variation.price, e
                        )
                    }),
                    unit: "IDK".into(),
                };
                let format = variation
                    .downloadable
                    .iter()
                    .map(|downloadable| {
                        downloadable
                            .no_musics
                            .iter()
                            .map(|no_music| {
                                let name = no_music.name.clone().into();
                                let url = no_music.url.clone();

                                NameWithUrl { name, url }
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                let mut ff = vec![];
                for f in format {
                    if !f.is_empty() {
                        let fff = f.clone();
                        for f in fff {
                            ff.push(f);
                        }
                    }
                }

                let mut format = None;

                if let Some(val) = ff.first() {
                    format = Some(val.name.name.clone());
                } else {
                    format = None;
                }

                DownloadInfo {
                    name,
                    price,
                    variation: None,
                    format,
                    // TODO
                    size: None,
                }
            })
            .collect();

        ItemMetadata {
            item,
            author,
            hearts,
            price,
            category,
            images,
            downloads,
            // TODO
            badges: Default::default(),
            // TODO
            description: "".to_string(),
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

impl Into<NameWithTranslation> for String {
    fn into(self) -> NameWithTranslation {
        NameWithTranslation {
            name: self.clone(),
            name_translated: self,
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
