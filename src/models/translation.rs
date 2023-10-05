use std::io::Write;
use std::sync::{Arc, Mutex, RwLock};

use lingual::{blocking, Lang};
use serde::{Deserialize, Serialize};

use crate::debug;
use crate::zaphkiel::cache::Cache;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TranslationError {
    TargetLangMismatch,
    ParseIntErr,
    HttpErr(String),
    UrlParseErr,
    JsonParseErr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlTranslationCTX {
    original_text: String,
    left: String,
    url: String,
    right: String,
    tled: String,
}

// Todo: refactor this
#[inline(always)]
pub fn translate(
    text: impl AsRef<str>,
    to_lang: Lang,
    cache: Option<Arc<RwLock<Cache>>>,
    ctxs: Option<Arc<Mutex<Vec<UrlTranslationCTX>>>>,
) -> Result<String, TranslationError> {
    let original_text = text.as_ref();
    if original_text.is_empty() {
        return Ok("".to_string());
    }
    let text = encode(original_text);
    if text.contains("http") {
        let (left, url, right) = handle_http(text.clone(), to_lang, cache.clone(), ctxs.clone())?;
        let tled_left = translate(left.clone(), to_lang, cache.clone(), ctxs.clone())?;
        let tled_right = translate(right.clone(), to_lang, cache.clone(), ctxs.clone())?;
        let tled = format!("{}{}{}", tled_left, url, tled_right);

        cache
            .clone()
            .unwrap()
            .write()
            .unwrap()
            .add(original_text.to_string(), tled.clone());
        cache
            .clone()
            .unwrap()
            .write()
            .unwrap()
            .add(url.clone(), url.clone());
        cache
            .clone()
            .unwrap()
            .write()
            .unwrap()
            .add(left.clone(), tled_left.clone());
        cache
            .clone()
            .unwrap()
            .write()
            .unwrap()
            .add(right.clone(), tled_right.clone());

        return Ok(format!("{} {} {}", left, url, right));
    }

    if let Some(cache) = &cache {
        if let Some(translation) = cache.read().unwrap().get(&text) {
            return Ok(decode(translation));
        }
    }
    let translation =
        blocking::translate(text.clone(), None, Some(to_lang)).map_err(|e| match e {
            lingual::Errors::HttpErr(e) => TranslationError::HttpErr(e),
            lingual::Errors::UrlParseErr => TranslationError::UrlParseErr,
            lingual::Errors::JsonParseErr => TranslationError::JsonParseErr,
            lingual::Errors::ParseIntErr => TranslationError::ParseIntErr,
        })?;
    if translation.target_lang() != to_lang {
        return Err(TranslationError::TargetLangMismatch);
    }
    if let Some(cache) = cache {
        cache
            .write()
            .unwrap()
            .add(text, translation.text().to_string());
    }
    Ok(decode(translation.text()))
}

#[inline(always)]
pub fn encode(text: impl AsRef<str>) -> String {
    let text = text.as_ref();
    let text = text.replace('\t', "{TAB}");
    let text = text.replace('\u{3000}', "{U3000}");
    let text = text.trim();

    text.into()
}

#[inline(always)]
pub fn decode(text: impl AsRef<str>) -> String {
    let text = text.as_ref();
    let text = text.replace("{TAB}", "\t");
    let text = text.replace("{U3000}", "\u{3000}");

    text
}

#[inline(always)]
pub fn handle_http(
    text: String,
    to_lang: Lang,
    cache: Option<Arc<RwLock<Cache>>>,
    ctxs: Option<Arc<Mutex<Vec<UrlTranslationCTX>>>>,
) -> Result<(String, String, String), TranslationError> {
    let original_text = text.clone();
    let (left, right) = text.split_once("http").unwrap_or((&text, ""));
    let pos = right
        .find(|c: char| !{
            c == '&'
                || c == '$'
                || c == '+'
                || c == '.'
                || c == ','
                || c == '/'
                || c == ':'
                || c == ';'
                || c == '='
                || c == '_'
                || c == '?'
                || c == '@'
                || c == '#'
                || c == '-'
                || c.is_alphanumeric()
        })
        .unwrap_or(right.len());
    let (url, right) = right.split_at(pos);
    let url = format!("http{}", url);

    let (left, right) = (
        translate(left, to_lang, cache.clone(), ctxs.clone())?,
        translate(right, to_lang, cache.clone(), ctxs.clone())?,
    );

    let tled = format!("{} {} {}", &left, &url, &right);

    let ctx = UrlTranslationCTX {
        original_text: original_text.to_string(),
        left: left.clone(),
        url: url.clone(),
        right: right.clone(),
        tled: tled.clone(),
    };

    debug!(&ctx);

    if let Some(cache) = cache {
        cache
            .write()
            .unwrap()
            .add(original_text.to_string(), ctx.tled.clone());
        cache.write().unwrap().add(url.clone(), url.clone());
    }

    if let Some(ctxs) = ctxs {
        ctxs.lock().unwrap().push(ctx.clone());
    }

    return Ok((left, url, right));
}
