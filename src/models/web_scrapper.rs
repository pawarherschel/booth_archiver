use std::fs;
use std::sync::{Arc, RwLock};

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use ureq::{Agent, AgentBuilder};

use crate::zaphkiel::cache::{Cache, HtmlCacheStats};

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebScraper {
    client: Agent,
    cache: Arc<RwLock<Cache>>,
    cookie: String,
}

impl WebScraper {
    pub fn dump_cache(&self) {
        self.cache.clone().read().unwrap().dump();
    }
}

impl WebScraper {
    pub fn new(cookie: String, adult: bool) -> Self {
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
            AppleWebKit/537.36 (KHTML, like Gecko) \
            Chrome/108.0.0.0 \
            Safari/537.36";

        let client = AgentBuilder::new().user_agent(user_agent).build();

        let session_cookie = format!("_plaza_session_nktz7u={}; Secure", cookie);
        let adult_cookie = format!("adult={}; Secure", if adult { "t" } else { "f" });
        let cookie = format!("{}; {}", session_cookie, adult_cookie);

        let cache = if fs::metadata("cache.json").is_ok() {
            Arc::new(RwLock::new(Cache::new_from_file("cache.json".into())))
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
    pub fn get_cache_stats(&self) -> String {
        let stats = HtmlCacheStats {
            ..self
                .cache
                .clone()
                .read()
                .unwrap()
                .stats
                .clone()
                .read()
                .unwrap()
                .clone()
        };

        format!("{:#?}", stats)
    }

    pub fn get_one(&self, url: String) -> Result<String, Box<dyn std::error::Error>> {
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

    pub fn get_many(&self, urls: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let pb = ProgressBar::new(urls.len() as u64);

        let pb_style = ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec})",
            )?
            .progress_chars("#>-");
        pb.set_style(pb_style);
        pb.tick();

        let htmls = urls
            .par_iter()
            .progress_with(pb)
            .map(|url| self.get_one(url.clone()).unwrap())
            .collect::<Vec<_>>();

        // for (i, url) in urls.iter().enumerate() {
        //     let html = self.get_one(url.clone())?;
        //     htmls.push(html);
        //
        //     pb.set_message(&format!("{} downloaded", url));
        //     pb.inc(1);
        // }

        Ok(htmls)
    }
}
