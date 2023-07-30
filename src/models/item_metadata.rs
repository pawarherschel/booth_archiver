#[derive(Debug, Default, Clone)]
pub struct Item {
    pub item: ItemInfo,
    pub author: NameWithUrl,
    pub hearts: u32,
    pub price: NumberWithUnit,
    pub category: CategoryInfo,
    pub images: Vec<NameWithUrl>,
    pub downloads: Vec<DownloadInfo>,
}

#[derive(Debug, Default, Clone)]
pub struct ItemInfo {
    pub name: NameWithUrl,
    pub id: u32,
}

#[derive(Debug, Default, Clone)]
pub struct CategoryInfo {
    pub category: NameWithUrl,
    pub subcategory: Option<NameWithUrl>,
}

#[derive(Debug, Default, Clone)]
pub struct DownloadInfo {
    pub names: NameWithUrl,
    pub price: NumberWithUnit,
    pub variation: Option<NameWithTranslation>,
    pub format: Option<String>,
    pub size: Option<NumberWithUnit>,
}

#[derive(Debug, Default, Clone)]
pub struct NameWithTranslation {
    pub name: String,
    pub name_translated: String,
}

#[derive(Debug, Default, Clone)]
pub struct NameWithUrl {
    pub name: NameWithTranslation,
    pub url: String,
}

#[derive(Debug, Default, Clone)]
pub struct Badges {
    pub vrchat: bool,
    pub adult: bool,
}

#[derive(Debug, Default, Clone)]
pub struct NumberWithUnit {
    pub number: f64,
    pub unit: String,
}
