use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};

use crate::time_it;

/// generic cache that stores a key-value pair
/// It is safe to use this cache from multiple threads if you wrap it in an Arc<RwLock<_>>
#[derive(Debug, Default, Clone)]
pub struct Cache {
    pub cache: HashMap<String, String>,
    pub stats: Arc<RwLock<HtmlCacheStats>>,
    accesses: u64,
    path_to_cache: PathBuf,
}

/// Stats for the cache
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HtmlCacheStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Cache {
    /// Create a new cache with default values
    pub fn new() -> Self {
        Self::new_with_path("cache.json".into())
    }

    /// Create a new cache with a custom path
    pub fn new_with_path(path_to_cache: PathBuf) -> Self {
        Self {
            path_to_cache,
            ..Self::default()
        }
    }

    /// Create a new cache and uses the cache from the given path
    pub fn new_from_file(cache_location: PathBuf) -> Self {
        let mut new = Self::new_with_path(cache_location.clone());

        new.pump_from_file(cache_location);

        new
    }
}

impl Cache {
    /// Add a key-value pair to the cache
    pub fn add(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
        self.accesses += 1;
        if self.accesses % 100 == 0 {
            self.dump();
        }
    }

    /// Get a value from the cache
    pub fn get(&self, key: &String) -> Option<String> {
        if let Some(value) = self.cache.get(&*key) {
            self.hit();
            Some(value.clone())
        } else {
            self.miss();
            None
        }
    }
}

impl Cache {
    /// increments the cache hit counter
    pub fn hit(&self) {
        self.stats.clone().write().unwrap().cache_hits += 1;
    }

    /// increments the cache miss counter
    pub fn miss(&self) {
        self.stats.clone().write().unwrap().cache_misses += 1;
    }
}

impl Cache {
    /// pump the cache from the cache file
    pub fn pump(&mut self) {
        self.pump_from_file(self.path_to_cache.clone());
    }

    /// dump the cache to the cache file
    pub fn dump(&self) {
        self.dump_to_file(self.path_to_cache.clone());
    }

    /// pump the cache from the given cache file
    pub fn pump_from_file(&mut self, cache_location: PathBuf) {
        let cache = time_it!("reading from cache file" =>
        fs::read_to_string(&cache_location)
            .unwrap_or_else(|_| panic!("{} not found",
                    cache_location
                        .absolutize()
                        .expect("failed to absolutize path")
                        .to_str()
                        .expect("failed to convert path to str")))
        );

        let cache: HashMap<String, String> = time_it!("converting to hashmap from string" =>
            serde_json::from_str(&cache)
                .expect("Failed to parse cache.json, \
                cache.json exists but the json data is invalid")
        );

        self.cache = cache;
    }

    /// dump the cache to the given cache file
    pub fn dump_to_file(&self, cache_location: PathBuf) {
        let cache =
            serde_json::to_string(&self.cache).expect("failed to serialize from hashmap to json");

        let path = cache_location.to_str().unwrap_or_else(|| {
            panic!(
                "{} not found",
                cache_location
                    .absolutize()
                    .expect("failed to absolutize path")
                    .to_str()
                    .expect("failed to convert path to str")
            )
        });

        fs::write(path, cache)
            .unwrap_or_else(|e| panic!("failed to write to cache file because of error: {}", e));
    }
}
