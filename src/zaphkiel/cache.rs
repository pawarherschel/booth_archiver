use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};

use crate::time_it;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub struct Cache {
    pub cache: HashMap<String, String>,
    pub stats: RefCell<HtmlCacheStats>,
    accesses: u64,
    path_to_cache: PathBuf,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub struct HtmlCacheStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Cache {
    pub fn new() -> Self {
        Self::new_with_path("cache.json".into())
    }

    pub fn new_with_path(path_to_cache: PathBuf) -> Self {
        Self {
            path_to_cache,
            ..Self::default()
        }
    }

    pub fn new_from_file(cache_location: PathBuf) -> Self {
        let mut new = Self::new_with_path(cache_location.clone());

        new.pump_from_file(cache_location);

        new
    }
}

impl Cache {
    pub fn add(&mut self, key: String, value: String) {
        self.cache.insert(key, value);
        self.accesses += 1;
        if self.accesses % 100 == 0 {
            self.dump();
        }
    }
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
    pub fn hit(&self) {
        self.stats.borrow_mut().cache_hits += 1;
    }
    pub fn miss(&self) {
        self.stats.borrow_mut().cache_misses += 1;
    }
}

impl Cache {
    pub fn pump(&mut self) {
        self.pump_from_file("cache.json".into());
    }
    pub fn dump(&self) {
        self.dump_to_file("cache.json".into());
    }

    pub fn pump_from_file(&mut self, cache_location: PathBuf) {
        println!("pumping from {:?}", &cache_location);
        let cache = time_it!("\treading from cache file" =>
        fs::read_to_string(&cache_location)
            .unwrap_or_else(|_| panic!("{} not found",
                    cache_location
                        .absolutize()
                        .expect("failed to absolutize path")
                        .to_str()
                        .expect("failed to convert path to str")))
        );

        let cache: HashMap<String, String> = time_it!("\tconverting to hashmap from string" =>
            serde_json::from_str(&cache)
                .expect("Failed to parse cache.json, \
                cache.json exists but the json data is invalid")
        );

        self.cache = cache;
        println!("pumped");
    }
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
