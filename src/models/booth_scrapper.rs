use crate::zaphkiel::html_cache::HtmlCache;
use crate::zaphkiel::html_cache_stats::HtmlCacheStats;
use reqwest::{Client, Url};
use scraper::Html;
use tokio::io::AsyncReadExt;
use tokio::task;

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebScraper {
    pub client: Client,
    cookie: String,
    cache: HtmlCache,
}

impl WebScraper {
    pub fn new(cookie: String, adult: bool) -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();

        let session_cookie = format!("_plaza_session_nktz7u={}; Secure", cookie);
        let adult_cookie = format!("adult={}; Secure", if adult { "t" } else { "f" });
        let cookie = format!("{}; {}", session_cookie, adult_cookie);

        let cache = HtmlCache::new();

        Self {
            client,
            cookie,
            cache,
        }
    }
}

impl WebScraper {
    pub async fn get_one(&self, url: &str) -> Result<Html, Box<dyn std::error::Error>> {
        let url = Url::parse(url)?;

        if let Some(html) = self.cache.read().await.get(&url) {
            self.stats.write().await.cache_hits += 1;
            return Ok(html.clone());
        }

        let res = self
            .client
            .get(url.clone())
            .header("Cookie", self.cookie.clone())
            .send()
            .await?;

        let html = Html::parse_document(&res.text().await?);

        self.add_to_cache(url.clone(), html.clone());

        Ok(html)
    }

    pub async fn get_many(&self, urls: &[&str]) -> Result<Vec<Html>, Box<dyn std::error::Error>> {
        let mut htmls = Vec::new();

        for url in urls {
            let html = self.get_one(url).await?;
            htmls.push(html);
        }

        Ok(htmls)
    }
}
