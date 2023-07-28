use reqwest::Url;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct HtmlCache {
    pub cache: Arc<RwLock<HashMap<Url, Html>>>,
    pub stats: Arc<RwLock<HtmlCacheStats>>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[allow(dead_code)]
pub struct HtmlCacheStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl HtmlCache {
    pub fn new() -> Self {
        match (
            fs::read_to_string("cache.json"),
            fs::read_to_string("stats.json"),
        ) {
            (Ok(cache), Ok(stats)) => {
                let cache: HashMap<String, String> = serde_json::from_str(&cache).unwrap();
                let stats: HtmlCacheStats = serde_json::from_str(&stats).unwrap();

                let cache: HashMap<Url, Html> = cache
                    .into_iter()
                    .map(|(k, v)| (Url::parse(&k).unwrap(), Html::parse_document(&v)))
                    .collect();

                Self {
                    cache: Arc::new(RwLock::new(cache)),
                    stats: Arc::new(RwLock::new(stats)),
                }
            }
            _ => Self::default(),
        }

        Self::default()
    }
}

impl HtmlCache {
    pub async fn add(&self, url: Url, html: Html) {
        self.cache.write().await.insert(url, html);
        self.miss().await;
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
        let cache = fs::read_to_string("cache.json").unwrap();
        let stats = fs::read_to_string("stats.json").unwrap();

        let cache: HashMap<String, String> = serde_json::from_str(&cache).unwrap();
        let stats: HtmlCacheStats = serde_json::from_str(&stats).unwrap();

        let cache: HashMap<Url, Html> = cache
            .into_iter()
            .map(|(k, v)| (Url::parse(&k).unwrap(), Html::parse_document(&v)))
            .collect();

        *self.cache.write().await = cache;
        *self.stats.write().await = stats;
    }
    pub async fn dump(&self) {
        let serialize_friendly_map: HashMap<String, String> = {
            let cache = self.cache.read().await.deref().clone();
            cache
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.html()))
                .collect()
        };

        let stats = self.stats.read().await.deref().clone();

        let cache = serde_json::to_string(&serialize_friendly_map).unwrap();
        let stats = serde_json::to_string(&stats).unwrap();

        fs::write("cache.json", cache).unwrap();
        fs::write("stats.json", stats).unwrap();
    }
}
