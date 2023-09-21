use std::path::PathBuf;

use lazy_static::lazy_static;
use path_absolutize::Absolutize;

use super::super::models::web_scrapper::WebScraper;

lazy_static! {
    /// The cookie for the program.
    pub static ref COOKIE: String = {
        let cookie_file_path = PathBuf::from("cookie.txt");
        std::fs::read_to_string(
            cookie_file_path.clone()
        ).unwrap_or_else(|e| {
            panic!(
                "expecting cookie to be in {}, because of error: {}",
                cookie_file_path
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
}
