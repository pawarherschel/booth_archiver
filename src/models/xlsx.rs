use rust_xlsxwriter::{ColNum, Url, Workbook, Worksheet, XlsxError};

use crate::api_structs::items::ItemApiResponse;
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
        download_links: downloads_links,
        markdown,
    } = item.to_owned();

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
    worksheet.write(row, Headers::ImagesNumber.into(), image_urls.len() as u32)?;
    worksheet.write(row, Headers::ImagesURLs.into(), image_urls.join("\n"))?;
    worksheet.write(
        row,
        Headers::DownloadNumber.into(),
        downloads_links.len() as u32,
    )?;
    worksheet.write(
        row,
        Headers::DownloadsLinks.into(),
        downloads_links.join("\n"),
    )?;
    worksheet.write(row, Headers::Markdown.into(), markdown)?;

    Ok(())
}

pub fn write_all(worksheet: &mut Worksheet, items: Vec<ItemApiResponse>) {
    items
        .iter()
        .map(|item| item.to_owned().into())
        .enumerate()
        .for_each(|(idx, item)| write_row(&item, worksheet, idx as u32 + 1).unwrap());
}

pub fn format_cols(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    let last: ColNum = Headers::Markdown.into();
    let last = last as u32;

    worksheet.autofilter(0, 0, last, 0)?;

    Ok(())
}

pub fn save_book(workbook: &mut Workbook, path: &'static str) {
    match workbook.save(path) {
        Ok(_) => {
            dbg!("saved");
        }
        Err(e) => match e {
            XlsxError::IoError(e) => println!(
                "io error: {}\n\
                Did you check if the file is already open in excel?",
                e
            ),
            _ => {
                dbg!("error: {}", e);
            }
        },
    };
}
