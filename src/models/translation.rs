use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranslationError {
    TargetLangMismatch,
    ParseIntErr,
    HttpErr(String),
    UrlParseErr,
    JsonParseErr(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlTranslationCTX {
    original_text: String,
    left: String,
    url: String,
    right: String,
    tled: String,
}

#[inline]
pub fn encode(text: impl AsRef<str>) -> String {
    let text = text.as_ref();
    let text = text.replace('\t', "{TAB}");
    let text = text.replace('\u{3000}', " ");
    let text = text.trim();

    text.into()
}

#[inline]
pub fn decode(text: impl AsRef<str>) -> String {
    let text = text.as_ref();
    text.replace("{TAB}", "\t")
}

#[inline]
#[must_use]
pub fn handle_http(text: &str) -> (String, String, String) {
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
    let url = format!("http{url}");

    let (left, right) = (left.to_string(), right.to_string());

    (left, url, right)
}
