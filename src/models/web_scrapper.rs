use std::fs;
use std::sync::{Arc, RwLock};

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use ureq::{Agent, AgentBuilder};

use crate::zaphkiel::cache::{Cache, HtmlCacheStats};
use crate::zaphkiel::utils::get_pb;

/// Basic web scraper that uses a cache to avoid downloading the same page twice.
#[derive(Debug)]
pub struct WebScraper {
    client: Agent,
    cache: Arc<RwLock<Cache>>,
    cookie: String,
}

impl WebScraper {
    /// Dump the cache to a file.
    pub fn dump_cache(&self) {
        self.cache.clone().read().unwrap().dump();
    }
}

impl WebScraper {
    /// Create a new web scraper.
    ///
    /// # Arguments
    ///
    /// * `cookie` - The cookie to use for the requests.
    /// * `adult` - Whether to use the adult cookie or not.
    pub fn new(cookie: String, adult: bool) -> Self {
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
            AppleWebKit/537.36 (KHTML, like Gecko) \
            Chrome/108.0.0.0 \
            Safari/537.36";

        let client = AgentBuilder::new().user_agent(user_agent).build();

        let session_cookie = format!("_plaza_session_nktz7u={}; Secure", cookie);
        let adult_cookie = format!("adult={}; Secure", if adult { "t" } else { "f" });
        let cookie = format!("{}; {}", session_cookie, adult_cookie);

        let cache = if fs::metadata("cache.ron").is_ok() {
            Arc::new(RwLock::new(Cache::new_from_file("cache.ron".into())))
        } else {
            Arc::new(RwLock::new(Cache::new()))
        };

        Self {
            client,
            cookie,
            cache,
        }
    }
}

impl WebScraper {
    /// Get the cache stats.
    pub fn get_cache_stats(&self) -> String {
        let stats = HtmlCacheStats {
            ..self.cache.clone().read().unwrap().get_stats()
        };

        format!("{:#?}", stats)
    }

    /// Get the cache misses.
    pub fn get_cache_misses(&self) -> String {
        let misses = self.cache.clone().read().unwrap().get_misses();

        format!("{:#?}", misses)
    }

    /// Get a single page.
    #[allow(clippy::result_large_err)]
    pub fn get_one(&self, url: String) -> Result<String, ureq::Error> {
        if let Some(html) = self.cache.clone().read().unwrap().get(&url) {
            return Ok(html);
        }

        let res = self
            .client
            .get(&url)
            .set("Cookie", &self.cookie.clone())
            .call()?
            .into_string()?;

        self.cache.clone().write().unwrap().add(url, res.clone());

        Ok(res)
    }

    /// Get multiple pages, in parallel.
    pub fn get_many(
        &self,
        urls: Vec<String>,
        msg: &'static str,
    ) -> Vec<Result<String, ureq::Error>> {
        let htmls = urls
            .par_iter()
            .progress_with(get_pb(urls.len() as u64, msg))
            .map(|url| self.get_one(url.clone()))
            .collect::<Vec<_>>();
        htmls
    }

    #[allow(clippy::result_large_err)]
    pub fn get_json(&self, _url: String) -> Result<String, ureq::Error> {
        todo!()
    }
}
