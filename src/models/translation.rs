use crate::zaphkiel::cache::Cache;
use lingual::{blocking, Lang};
use std::sync::{Arc, RwLock};

pub fn translate(
    text: impl AsRef<str>,
    to_lang: Lang,
    cache: Option<Arc<RwLock<Cache>>>,
) -> Result<String, lingual::Errors> {
    if let Some(cache) = &cache {
        if let Some(translation) = cache.read().unwrap().get(&text.as_ref().into()) {
            return Ok(translation);
        }
    }
    let text = text.as_ref();
    let translation = blocking::translate(text, None, Some(to_lang))?;
    if let Some(cache) = cache {
        cache
            .write()
            .unwrap()
            .add(text.into(), translation.text().to_string());
    }
    Ok(translation.text().into())
}
