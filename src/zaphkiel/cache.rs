use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use path_absolutize::Absolutize;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::time_it;

/// generic cache that stores a key-value pair
/// It is safe to use this cache from multiple threads if you wrap it in an Arc<`RwLock`<_>>
#[derive(Debug, Default, Clone)]
pub struct Cache {
    cache: HashMap<String, String>,
    stats: Arc<RwLock<CacheStats>>,
    misses: Arc<RwLock<Vec<String>>>,
    hits: Arc<RwLock<Vec<String>>>,
    accesses: u64,
    path_to_cache: PathBuf,
}

/// Stats for the cache
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CacheStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_size: usize,
}

impl Cache {
    /// Create a new cache with a custom path
    pub fn new_with_path(path_to_cache: PathBuf) -> Self {
        let cache = match fs::metadata(&path_to_cache) {
            Ok(_) => {
                let abs_path = path_to_cache
                    .absolutize()
                    .expect("failed to absolutize path");
                let abs_path = abs_path.to_str().expect("failed to convert path to str");

                let cache = fs::read_to_string(&path_to_cache)
                    .unwrap_or_else(|_| panic!("{} not found", abs_path));

                let cache: HashMap<String, String> = ron::from_str(&cache).unwrap_or_else(|e| {
                    panic!(
                        "Failed to parse {}, {} exists but the ron data is invalid\n\
                        Failed with error: {}",
                        abs_path, abs_path, e
                    )
                });

                cache
            }
            Err(_) => HashMap::new(),
        };

        Self {
            cache,
            path_to_cache,
            ..Self::default()
        }
    }
}

impl Cache {
    #[inline]
    /// Add a key-value pair to the cache
    pub fn add(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
        self.accesses += 1;
        if self.accesses % 100 == 0 {
            self.dump();
        }
    }

    #[inline]
    /// Get a value from the cache
    #[must_use]
    pub fn get(&self, key: &String) -> Option<String> {
        self.cache.get(key).map_or_else(
            || {
                self.miss();
                self.misses.write().unwrap().push(key.clone());
                None
            },
            |value| {
                self.hit();
                self.hits.write().unwrap().push(key.clone());
                Some(value.clone())
            },
        )
    }
}

impl Cache {
    #[inline]
    /// increments the cache hit counter
    pub fn hit(&self) {
        self.stats.clone().write().unwrap().cache_hits += 1;
    }

    #[inline]
    /// increments the cache miss counter
    pub fn miss(&self) {
        self.stats.clone().write().unwrap().cache_misses += 1;
    }
}

impl Cache {
    /// pump the cache from the cache file
    pub fn pump(&mut self) {
        self.pump_from_file(&self.path_to_cache.clone());
    }

    /// dump the cache to the cache file
    pub fn dump(&self) {
        self.dump_to_file(&self.path_to_cache);
    }

    /// pump the cache from the given cache file
    pub fn pump_from_file(&mut self, cache_location: &PathBuf) {
        let abs_path = cache_location
            .absolutize()
            .expect("failed to absolutize path");
        let abs_path = abs_path.to_str().expect("failed to convert path to str");
        let cache = time_it!("reading from cache file" =>
        fs::read_to_string(cache_location)
            .unwrap_or_else(|_| panic!("{} not found", abs_path))
        );

        let cache: HashMap<String, String> = time_it!("converting to hashmap from string" =>
            ron::from_str(&cache)
                .unwrap_or_else(|e| panic!("Failed to parse {}, \
                {} exists but the ron data is invalid\n\
                failed with error: {}", abs_path, abs_path, e))
        );

        self.cache = cache;
    }

    /// dump the cache to the given cache file
    pub fn dump_to_file(&self, cache_location: &PathBuf) {
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
    #[must_use]
    pub fn get_stats(&self) -> CacheStats {
        self.stats.write().unwrap().cache_size = self.cache.len();
        self.stats.clone().read().unwrap().clone()
    }

    /// get the keys which caused misses
    #[must_use]
    pub fn get_misses(&self) -> Vec<String> {
        self.misses.clone().read().unwrap().clone()
    }

    /// get the keys which caused hits
    #[must_use]
    pub fn get_hits(&self) -> Vec<String> {
        self.hits.clone().read().unwrap().clone()
    }

    /// get the number of times the cache was accessed
    #[must_use]
    pub const fn get_accesses(&self) -> u64 {
        self.accesses
    }

    /// get the path to the cache file
    #[must_use]
    pub fn get_path_to_cache(&self) -> PathBuf {
        self.path_to_cache.clone()
    }

    pub fn keys(&self) -> impl Iterator<Item = String> + '_ {
        self.cache.keys().cloned()
    }

    /// clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn capacity(&self) -> usize {
        self.cache.capacity()
    }
}
