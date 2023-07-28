use crate::zaphkiel::html_cache_stats::HtmlCacheStats;
use reqwest::Url;
use scraper::Html;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
#[allow(dead_code)]
pub struct HtmlCache {
    pub cache: Arc<RwLock<HashMap<Url, Html>>>,
    pub stats: Arc<RwLock<HtmlCacheStats>>,
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct HtmlCacheStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub add_to_cache_retries: u64,
}

impl HtmlCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HtmlCacheStats::default())),
        }
    }
}

impl HtmlCache {
    pub async fn add(&self, url: Url, html: Html) {
        self.cache.write().await.insert(url, html).unwrap();
        self.miss().await;
    }
}

impl HtmlCache {
    pub async fn hit(&self) {
        self.stats.write().await.cache_hits += 1;
    }
    pub async fn miss(&self) {
        self.stats.write().await.cache_misses += 1;
    }
    pub async fn retry(&self) {
        self.stats.write().await.add_to_cache_retries += 1;
    }
}
