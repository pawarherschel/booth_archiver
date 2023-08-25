use rust_xlsxwriter::{ColNum, Worksheet};

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
            Headers::Price => 10,
            Headers::Currency => 11,
            Headers::Hearts => 12,
            Headers::ImagesNumber => 13,
            Headers::ImagesURLs => 14,
            Headers::DownloadNumber => 15,
            Headers::DownloadsLinks => 16,
            Headers::Markdown => 17,
        }
    }
}

pub fn write_headers(worksheet: &mut Worksheet) -> Result<(), rust_xlsxwriter::XlsxError> {
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

pub fn write_row(
    item: &ItemRow,
    worksheet: &mut Worksheet,
    row: u32,
) -> Result<(), rust_xlsxwriter::XlsxError> {
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
        price,
        currency,
        hearts,
        image_urls,
        downloads_links,
        markdown,
    } = item.to_owned();

    worksheet.write(row, Headers::ItemName.into(), item_name)?;
    worksheet.write(
        row,
        Headers::ItemNameTranslated.into(),
        item_name_translated,
    )?;
    worksheet.write(row, Headers::ItemLink.into(), item_link)?;
    worksheet.write(row, Headers::AuthorName.into(), author_name)?;
    worksheet.write(
        row,
        Headers::AuthorNameTranslated.into(),
        author_name_translated,
    )?;
    worksheet.write(row, Headers::AuthorLink.into(), author_link)?;
    worksheet.write(row, Headers::PrimaryCategory.into(), primary_category)?;
    worksheet.write(row, Headers::SecondaryCategory.into(), secondary_category)?;
    worksheet.write(row, Headers::VRChat.into(), vrchat)?;
    worksheet.write(row, Headers::Adult.into(), adult)?;
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
