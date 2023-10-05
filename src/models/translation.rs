use std::sync::{Arc, RwLock};

use lingual::{blocking, Lang};
use serde::{Deserialize, Serialize};

use crate::zaphkiel::cache::Cache;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TranslationError {
    TargetLangMismatch,
    ParseIntErr,
    HttpErr(String),
    UrlParseErr,
    JsonParseErr,
}

#[inline(always)]
pub fn translate(
    text: impl AsRef<str>,
    to_lang: Lang,
    cache: Option<Arc<RwLock<Cache>>>,
) -> Result<String, TranslationError> {
    if let Some(cache) = &cache {
        if let Some(translation) = cache.read().unwrap().get(&text.as_ref().into()) {
            return Ok(translation);
        }
    }
    let text = text.as_ref();
    let translation = blocking::translate(text, None, Some(to_lang)).map_err(|e| match e {
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
            .add(text.into(), translation.text().to_string());
    }
    Ok(translation.text().into())
}
