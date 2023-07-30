use lazy_static::lazy_static;
use path_absolutize::Absolutize;

use super::super::models::config::Config;
use super::super::models::web_scrapper::WebScraper;
use super::super::time_it;

lazy_static! {
    /// The config for the program.
    pub static ref CONFIG: Config = time_it!("loading config" => Config::get());

    /// The cookie for the program.
    pub static ref COOKIE: String = {
        std::fs::read_to_string(
            CONFIG
                .cookie_file
                .as_ref()
                .expect("failed to build Path from PathBuf"),
        )
        .unwrap_or_else(|e| {
            panic!(
                "expecting cookie to be in {}, because of error: {}",
                CONFIG
                    .cookie_file
                    .as_ref()
                    .expect(
                        "failed to build Path from PathBuf, \
                        imagine panicking inside a panic lmao"
                    )
                    .absolutize()
                    .expect(
                        "failed to absolutize path from PathBuf, \
                        imagine panicking inside a panic lmao"
                    )
                    .to_str()
                    .expect("failed to convert path to str, imagine panicking inside a panic lmao"),
                e
            )
        })
    };
    pub static ref CLIENT: WebScraper = WebScraper::new(COOKIE.to_string(), true);
    pub static ref TEST_URLS: Vec<String> = {
        use super::super::temp::testing_urls::TESTING_URLS;
        TESTING_URLS.iter().map(|&s| s.to_owned()).collect()
    };
}
