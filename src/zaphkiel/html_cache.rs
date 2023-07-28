use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::sync::Arc;

use reqwest::Url;
use scraper::Html;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::time_it;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct HtmlCache {
    pub cache: Arc<RwLock<HashMap<Url, Html>>>,
    pub stats: Arc<RwLock<HtmlCacheStats>>,
    accesses: Arc<RwLock<u64>>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[allow(dead_code)]
pub struct HtmlCacheStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl HtmlCache {
    pub async fn new() -> Self {
        let default = Self::default();

        if fs::read_to_string("cache.json").is_ok() {
            default.pump().await;
        }

        default
    }
}

impl HtmlCache {
    pub async fn add(&self, url: Url, html: Html) {
        self.cache.write().await.insert(url, html);
        self.miss().await;
        *self.accesses.write().await += 1;
        if *self.accesses.read().await % 100 == 0 {
            self.dump().await;
        }
    }
    pub async fn get(&self, url: &Url) -> Option<Html> {
        if let Some(html) = self.cache.read().await.get(url) {
            self.hit().await;
            Some(html.clone())
        } else {
            self.miss().await;
            None
        }
    }
}

impl HtmlCache {
    pub async fn hit(&self) {
        self.stats.write().await.cache_hits += 1;
    }
    pub async fn miss(&self) {
        self.stats.write().await.cache_misses += 1;
    }

    pub async fn pump(&self) {
        println!("pumping");
        let cache =
            time_it!("\treading from cache file" => fs::read_to_string("cache.json").unwrap());

        let cache: HashMap<String, String> = time_it!("\tconverting to hashmap from string" => serde_json::from_str(&cache).unwrap());

        let cache: HashMap<Url, Html> = time_it!("\tconverting to in-memory representation of the cache"
            => cache
            .into_iter()
            .map(|(k, v)| (Url::parse(&k).unwrap(), Html::parse_document(&v)))
            .collect()
        );

        *self.cache.write().await = cache;
        println!("pumped");
    }
    pub async fn dump(&self) {
        let serialize_friendly_map: HashMap<String, String> = {
            let cache = self.cache.read().await.deref().clone();
            cache
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.html()))
                .collect()
        };

        let cache = serde_json::to_string(&serialize_friendly_map).unwrap();

        fs::write("cache.json", cache).unwrap()
    }
}
