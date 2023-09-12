use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use path_absolutize::Absolutize;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Deserialize, Serialize};

use crate::time_it;

/// generic cache that stores a key-value pair
/// It is safe to use this cache from multiple threads if you wrap it in an Arc<RwLock<_>>
#[derive(Debug, Default, Clone)]
pub struct Cache {
    pub cache: HashMap<String, String>,
    stats: Arc<RwLock<HtmlCacheStats>>,
    misses: Arc<RwLock<Vec<String>>>,
    accesses: u64,
    path_to_cache: PathBuf,
}

/// Stats for the cache
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HtmlCacheStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_size: usize,
}

impl Cache {
    /// Create a new cache with default values
    pub fn new() -> Self {
        Self::new_with_path("cache.ron".into())
    }

    /// Create a new cache with a custom path
    pub fn new_with_path(path_to_cache: PathBuf) -> Self {
        let cache = match fs::metadata(&path_to_cache) {
            Ok(_) => {
                let cache = fs::read_to_string(&path_to_cache).unwrap_or_else(|_| {
                    panic!(
                        "{} not found",
                        &path_to_cache
                            .absolutize()
                            .expect("failed to absolutize path")
                            .to_str()
                            .expect("failed to convert path to str")
                    )
                });

                let cache: HashMap<String, String> = ron::from_str(&cache).expect(
                    "Failed to parse cache.ron, \
                                cache.ron exists but the ron data is invalid",
                );

                cache
            }
            Err(_) => HashMap::new(),
        };

        Self {
            path_to_cache,
            cache,
            ..Cache::default()
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
        if let Some(value) = self.cache.get(key) {
            self.hit();
            Some(value.clone())
        } else {
            self.miss();
            self.misses.write().unwrap().push(key.clone());
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
            ron::from_str(&cache)
                .expect("Failed to parse cache.ron, \
                cache.ron exists but the ron data is invalid")
        );

        self.cache = cache;
    }

    /// dump the cache to the given cache file
    pub fn dump_to_file(&self, cache_location: PathBuf) {
        if self.get_stats().cache_misses == 0 {
            return;
        }

        let cache = to_string_pretty(&self.cache, PrettyConfig::default())
            .expect("failed to serialize from hashmap to ron");

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

        if fs::metadata(path).is_err() {
            File::create(path).unwrap_or_else(|err| {
                panic!("Failed to create file `{path}` due to error: {err}`")
            });
        }

        fs::write(path, cache)
            .unwrap_or_else(|e| panic!("failed to write to cache file because of error: {}", e));
    }
}

impl Cache {
    /// get the stats of the cache
    pub fn get_stats(&self) -> HtmlCacheStats {
        self.stats.write().unwrap().cache_size = self.cache.len();
        self.stats.clone().read().unwrap().clone()
    }

    /// get the keys which caused misses
    pub fn get_misses(&self) -> Vec<String> {
        self.misses.clone().read().unwrap().clone()
    }

    /// get the number of times the cache was accessed
    pub fn get_accesses(&self) -> u64 {
        self.accesses
    }

    /// get the path to the cache file
    pub fn get_path_to_cache(&self) -> PathBuf {
        self.path_to_cache.clone()
    }
}
