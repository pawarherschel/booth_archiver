use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// File where the cookie is stored
    #[arg(short, long)]
    pub cookie_file: Option<PathBuf>,
}

impl Config {
    pub fn get() -> Self {
        let cfg = Self::parse();

        if cfg.cookie_file.is_none() {
            Self {
                cookie_file: Some(PathBuf::from("cookie.txt")),
            }
        } else {
            cfg
        }
    }
}
