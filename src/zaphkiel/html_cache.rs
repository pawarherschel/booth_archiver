use reqwest::Url;
use scraper::Html;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
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
}

impl HtmlCache {
    pub fn new() -> Self {
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
}
