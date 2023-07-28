use crate::zaphkiel::html_cache::{HtmlCache, HtmlCacheStats};
use reqwest::{Client, Url};
use scraper::Html;

#[derive(Debug)]
#[allow(dead_code)]
pub struct WebScraper {
    pub client: Client,
    cookie: String,
    cache: HtmlCache,
}

impl WebScraper {
    pub async fn dump_cache(&self) {
        self.cache.dump().await;
    }
}

impl WebScraper {
    pub async fn new(cookie: String, adult: bool) -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();

        let session_cookie = format!("_plaza_session_nktz7u={}; Secure", cookie);
        let adult_cookie = format!("adult={}; Secure", if adult { "t" } else { "f" });
        let cookie = format!("{}; {}", session_cookie, adult_cookie);

        let cache = HtmlCache::new().await;

        Self {
            client,
            cookie,
            cache,
        }
    }
}

impl WebScraper {
    pub async fn get_cache_stats(&self) -> String {
        let stats = HtmlCacheStats {
            ..*self.cache.stats.read().await
        };

        format!("{:#?}", stats)
    }

    pub async fn get_one(&self, url: &str) -> Result<Html, Box<dyn std::error::Error>> {
        let url = Url::parse(url)?;

        if let Some(html) = self.cache.get(&url).await {
            return Ok(html);
        }

        let res = self
            .client
            .get(url.clone())
            .header("Cookie", self.cookie.clone())
            .send()
            .await?;

        let html = Html::parse_document(&res.text().await?);

        self.cache.add(url, html.clone()).await;

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
