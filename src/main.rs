use booth_archiver::models::config::Config;
use booth_archiver::models::web_scrapper::WebScraper;
use booth_archiver::time_it;
use clap::Parser;
use lazy_static::lazy_static;
use scraper::{Html, Selector};
use std::error::Error;

lazy_static! {
    pub static ref CONFIG: Config = {
        println!("Config::get()");
        Config::get()
    };
    pub static ref COOKIE: String = {
        println!("cookie_file: {:?}", CONFIG.cookie_file.as_ref().unwrap());
        std::fs::read_to_string(CONFIG.cookie_file.as_ref().unwrap()).unwrap()
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = time_it!("config test" => config_test().await)?;
    let _ = time_it!("blocking vec test" => blocking_vec_test())?;
    let _ = time_it!("vec test" => vec_test().await)?;
    let document = time_it!("reqwest test" => reqwest_test().await)?;
    let _ = time_it!("parsing test" => parsing_test(document).await)?;

    println!("cookie: {}", COOKIE.as_str());

    let client = WebScraper::new(COOKIE.clone(), true).await;

    let result = time_it!("get \"「ダリア」 Sunkiss'd Anubis Makeup Texture Set - VRChat\"" 
                => client.get_one("https://booth.pm/en/items/4954841").await)?;
    println!(
        "title: {}",
        result
            .select(&Selector::parse("title").unwrap())
            .next()
            .unwrap()
            .inner_html()
    );

    let many = time_it!("get many pages [google.com, youtube.com, reddit.com]" => 
        client.get_many(&["https://google.com", "https://youtube.com", "https://rust-lang.org"])
        .await)?;

    for one in many {
        println!(
            "title: {}",
            one.select(&Selector::parse("title").unwrap())
                .next()
                .unwrap()
                .inner_html()
                .trim()
        );
    }

    let _ = time_it!("get many cached pages [google.com, youtube.com, reddit.com]" => 
        client.get_many(&["https://google.com", "https://youtube.com", "https://rust-lang.org"])
        .await)?;

    println!("cache stats: {}", client.get_cache_stats().await);

    client.dump_cache().await;

    Ok::<(), Box<dyn Error>>(())
}

async fn config_test() -> Result<Config, Box<dyn Error>> {
    let config = Config::parse();

    Ok::<Config, Box<dyn Error>>(config)
}

fn blocking_vec_test() -> Result<Vec<i32>, Box<dyn Error>> {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();

    Ok::<Vec<i32>, Box<dyn Error>>(v2)
}

async fn vec_test() -> Result<Vec<i32>, Box<dyn Error>> {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();

    Ok::<Vec<i32>, Box<dyn Error>>(v2)
}

async fn reqwest_test() -> Result<String, Box<dyn Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    Ok::<String, Box<dyn Error>>(body)
}

async fn parsing_test(document: String) -> Result<String, Box<dyn Error>> {
    let document = Html::parse_document(&document);
    let selector = Selector::parse("title").unwrap();
    let title = document.select(&selector).next().unwrap().inner_html();
    let title = title.trim();

    Ok::<String, Box<dyn Error>>(title.to_string())
}
