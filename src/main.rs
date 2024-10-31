mod booth_scrapper;
mod cache;

use crate::cache::{Cache, KV};
use reqwest::get;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, trace};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
#[tracing::instrument(level = "trace")]
async fn main() {
    let fmt_layer = tracing_subscriber::fmt::Layer::default()
        // format::Pretty: Emits excessively pretty, multi-line logs, optimized
        // for human readability. This is primarily intended to be used in local
        // development and debugging, or for command-line applications, where
        // automated analysis and compact storage of logs is less of a priority
        // than readability and visual appeal. See here for sample output.
        .pretty()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true);

    tracing_subscriber::Registry::default()
        .with(fmt_layer)
        .init();

    if fs::metadata("cache").is_err() {
        trace!("creating directory cache since it doesnt exist");
    }

    let mut kv: Cache<'_, BTreeMap<Cow<str>, Cow<str>>> = Cache::new(&"cache/kv");

    kv.insert("key", "value");

    kv.persist();
}
