use std::cell::RefCell;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;

use crate::zaphkiel::cache::{Cache, HtmlCacheStats};

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebScraper {
    client: Client,
    cache: RefCell<Cache>,
    cookie: String,
    user_agent: String,
}

impl WebScraper {
    pub fn dump_cache(&self) {
        self.cache.borrow().dump();
    }
}

impl WebScraper {
    pub fn new(cookie: String, adult: bool) -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .expect("failed to build client");

        let session_cookie = format!("_plaza_session_nktz7u={}; Secure", cookie);
        let adult_cookie = format!("adult={}; Secure", if adult { "t" } else { "f" });
        let cookie = format!("{}; {}", session_cookie, adult_cookie);

        let cache = RefCell::new(Cache::new());

        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
            AppleWebKit/537.36 (KHTML, like Gecko) \
            Chrome/108.0.0.0 \
            Safari/537.36"
            .to_string();

        Self {
            client,
            cookie,
            cache,
            user_agent,
        }
    }
}

impl WebScraper {
    pub fn get_cache_stats(&self) -> String {
        let stats = HtmlCacheStats {
            ..self.cache.borrow().stats.borrow().clone()
        };

        format!("{:#?}", stats)
    }

    pub fn get_one(&self, url: String) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(html) = self.cache.borrow().get(&url) {
            return Ok(html);
        }

        let res = self
            .client
            .get(url.clone())
            .header("Cookie", self.cookie.clone())
            .header("User-Agent", self.user_agent.clone())
            .send()?
            .text()?;

        self.cache.borrow_mut().add(url, res.clone());

        Ok(res)
    }

    pub fn get_many(&self, urls: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut htmls = Vec::new();

        let pb = ProgressBar::new(urls.len() as u64);

        let pb_style = ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({msg})",
            )?
            .progress_chars("#>-");
        pb.set_style(pb_style);

        pb.tick();

        for (i, url) in urls.iter().enumerate() {
            let html = self.get_one(url.clone())?;
            htmls.push(html);

            pb.set_message(format!("{} items downloaded", i + 1));
            pb.inc(1);
        }

        pb.finish_with_message("All items downloaded!");

        Ok(htmls)
    }
}
