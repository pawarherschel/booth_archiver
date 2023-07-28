use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: Option<String>,

    /// File where the cookie is stored
    #[arg(short, long)]
    pub cookie_file: Option<PathBuf>,
}

impl Config {
    pub fn get() -> Self {
        let cfg = Self::parse();

        if cfg.cookie_file.is_none() {
            Self {
                name: cfg.name,
                cookie_file: Some(PathBuf::from("cookie.txt")),
            }
        } else {
            cfg
        }
    }
}
