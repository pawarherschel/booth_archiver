use std::fmt::Debug;

use rust_xlsxwriter::{ColNum, Url, Workbook, Worksheet, XlsxError};

use crate::debug;
use crate::models::item_row::ItemRow;

#[derive(Debug, Clone, Copy)]
enum Headers {
    ItemName,
    ItemNameTranslated,
    ItemLink,
    AuthorName,
    AuthorNameTranslated,
    AuthorLink,
    PrimaryCategory,
    SecondaryCategory,
    VRChat,
    Adult,
    Tags,
    Price,
    Currency,
    Hearts,
    ImagesNumber,
    ImagesURLs,
    DownloadNumber,
    DownloadsLinks,
    Markdown,
    MarkdownTranslated,
}

impl From<Headers> for ColNum {
    fn from(header: Headers) -> Self {
        match header {
            Headers::ItemName => 0,
            Headers::ItemNameTranslated => 1,
            Headers::ItemLink => 2,
            Headers::AuthorName => 3,
            Headers::AuthorNameTranslated => 4,
            Headers::AuthorLink => 5,
            Headers::PrimaryCategory => 6,
            Headers::SecondaryCategory => 7,
            Headers::VRChat => 8,
            Headers::Adult => 9,
            Headers::Tags => 10,
            Headers::Price => 11,
            Headers::Currency => 12,
            Headers::Hearts => 13,
            Headers::ImagesNumber => 14,
            Headers::ImagesURLs => 15,
            Headers::DownloadNumber => 16,
            Headers::DownloadsLinks => 17,
            Headers::Markdown => 18,
            Headers::MarkdownTranslated => 19,
        }
    }
}

pub fn write_headers(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    const ROW: u32 = 0;

    worksheet.write(ROW, Headers::ItemName.into(), "Item Name")?;
    worksheet.write(
        ROW,
        Headers::ItemNameTranslated.into(),
        "Item Name Translated",
    )?;
    worksheet.write(ROW, Headers::ItemLink.into(), "Item Link")?;
    worksheet.write(ROW, Headers::AuthorName.into(), "Author Name")?;
    worksheet.write(
        ROW,
        Headers::AuthorNameTranslated.into(),
        "Author Name Translated",
    )?;
    worksheet.write(ROW, Headers::AuthorLink.into(), "Author Link")?;
    worksheet.write(ROW, Headers::PrimaryCategory.into(), "Primary Category")?;
    worksheet.write(ROW, Headers::SecondaryCategory.into(), "Secondary Category")?;
    worksheet.write(ROW, Headers::VRChat.into(), "VRChat")?;
    worksheet.write(ROW, Headers::Adult.into(), "Adult")?;
    worksheet.write(ROW, Headers::Tags.into(), "Tags")?;
    worksheet.write(ROW, Headers::Price.into(), "Price")?;
    worksheet.write(ROW, Headers::Currency.into(), "Currency")?;
    worksheet.write(ROW, Headers::Hearts.into(), "Hearts")?;
    worksheet.write(ROW, Headers::ImagesNumber.into(), "Images Number")?;
    worksheet.write(ROW, Headers::ImagesURLs.into(), "Images URLs")?;
    worksheet.write(ROW, Headers::DownloadNumber.into(), "Download Number")?;
    worksheet.write(ROW, Headers::DownloadsLinks.into(), "Downloads Links")?;
    worksheet.write(ROW, Headers::Markdown.into(), "Markdown")?;
    worksheet.write(
        ROW,
        Headers::MarkdownTranslated.into(),
        "Markdown Translated",
    )?;

    Ok(())
}

pub fn write_row(item: &ItemRow, worksheet: &mut Worksheet, row: u32) -> Result<(), XlsxError> {
    let ItemRow {
        item_name,
        item_name_translated,
        item_link,
        author_name,
        author_name_translated,
        author_link,
        primary_category,
        secondary_category,
        vrchat,
        adult,
        tags,
        price,
        currency,
        hearts,
        image_urls,
        download_links,
        markdown,
        markdown_translated,
    } = item.to_owned();

    let item_name_translated = item_name_translated.unwrap_or_else(|| item_name.clone());
    let author_name_translated = author_name_translated.unwrap_or_else(|| author_name.clone());
    let markdown_translated = markdown_translated.unwrap_or_else(|| markdown.clone());

    worksheet.write(row, Headers::ItemName.into(), item_name)?;
    worksheet.write(
        row,
        Headers::ItemNameTranslated.into(),
        item_name_translated,
    )?;
    worksheet.write(row, Headers::ItemLink.into(), Url::new(item_link))?;
    worksheet.write(row, Headers::AuthorName.into(), author_name)?;
    worksheet.write(
        row,
        Headers::AuthorNameTranslated.into(),
        author_name_translated,
    )?;
    worksheet.write(row, Headers::AuthorLink.into(), Url::new(author_link))?;
    worksheet.write(row, Headers::PrimaryCategory.into(), primary_category)?;
    worksheet.write(row, Headers::SecondaryCategory.into(), secondary_category)?;
    worksheet.write_boolean(row, Headers::VRChat.into(), vrchat)?;
    worksheet.write_boolean(row, Headers::Adult.into(), adult)?;
    worksheet.write(row, Headers::Tags.into(), tags.join(", "))?;
    worksheet.write(row, Headers::Price.into(), price)?;
    worksheet.write(row, Headers::Currency.into(), currency)?;
    worksheet.write(row, Headers::Hearts.into(), hearts)?;
    worksheet.write(
        row,
        Headers::ImagesNumber.into(),
        u32::try_from(image_urls.len()).unwrap(),
    )?;
    worksheet.write(row, Headers::ImagesURLs.into(), image_urls.join("\n"))?;
    worksheet.write(
        row,
        Headers::DownloadNumber.into(),
        u32::try_from(download_links.len()).unwrap(),
    )?;
    worksheet.write(
        row,
        Headers::DownloadsLinks.into(),
        download_links.join("\n"),
    )?;
    worksheet.write(row, Headers::Markdown.into(), markdown)?;
    worksheet.write(row, Headers::MarkdownTranslated.into(), markdown_translated)?;

    Ok(())
}

pub fn write_all(worksheet: &mut Worksheet, items: &[ItemRow]) {
    items.iter().enumerate().for_each(|(idx, item)| {
        write_row(item, worksheet, u32::try_from(idx).unwrap() + 1).unwrap();
    });
}

pub fn format_cols(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    worksheet.autofilter(0, 0, 0, Headers::Markdown.into())?;

    Ok(())
}

pub fn save_book(workbook: &mut Workbook, path: &'static str) {
    match workbook.save(path) {
        Ok(()) => {
            debug!("saved");
        }
        Err(e) => match e {
            #[allow(unused_variables)]
            XlsxError::IoError(e) => panic!(
                "io error: {e}\n\
                Did you check if the file is already open in excel?"
            ),
            _ => {
                panic!("error: {e}");
            }
        },
    };
}
