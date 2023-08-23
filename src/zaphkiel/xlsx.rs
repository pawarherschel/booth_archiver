use rust_xlsxwriter::{Url, Worksheet};

use crate::models::item_row::ItemRow;

pub fn write_headers(worksheet: &mut Worksheet) -> Result<(), rust_xlsxwriter::XlsxError> {
    worksheet.write(0, 0, "Item Name")?;
    worksheet.write(0, 1, "Item Name Translated")?;
    worksheet.write(0, 2, "Item Link")?;
    worksheet.write(0, 3, "Author Name")?;
    worksheet.write(0, 4, "Author Name Translated")?;
    worksheet.write(0, 5, "Author Link")?;
    worksheet.write(0, 6, "Category")?;
    worksheet.write(0, 7, "VRChat Badge")?;
    worksheet.write(0, 8, "Adult Badge")?;
    worksheet.write(0, 9, "Price")?;
    worksheet.write(0, 10, "Currency")?;
    worksheet.write(0, 11, "Wishlist Count")?;
    worksheet.write(0, 12, "Images Number")?;
    worksheet.write(0, 13, "Images URLs")?;
    worksheet.write(0, 14, "Downloads Numbers")?;
    worksheet.write(0, 15, "Downloads Dict")?;
    worksheet.write(0, 16, "Downloads Links")?;
    worksheet.write(0, 17, "Downloads Names")?;
    worksheet.write(0, 18, "Downloads Variations")?;
    worksheet.write(0, 19, "Downloads Formats")?;
    worksheet.write(0, 20, "Downloads Sizes")?;
    worksheet.write(0, 21, "Downloads Units")?;
    worksheet.write(0, 22, "Downloads Markdown")?;

    Ok(())
}

pub fn write_row(
    item: &ItemRow,
    worksheet: &mut Worksheet,
    row: u32,
) -> Result<(), rust_xlsxwriter::XlsxError> {
    worksheet.write(row, 0, item.item_name.clone())?;
    worksheet.write(row, 1, item.item_name_translated.clone())?;
    worksheet.write(row, 2, Url::new(item.item_link.clone()))?;
    worksheet.write(row, 3, item.author_name.clone())?;
    worksheet.write(row, 4, item.author_name_translated.clone())?;
    worksheet.write(row, 5, Url::new(item.author_link.clone()))?;
    worksheet.write(row, 6, item.primary_category.clone())?;
    worksheet.write(row, 7, item.secondary_category.clone())?;
    worksheet.write(row, 8, item.vrchat)?;
    worksheet.write(row, 9, item.adult)?;
    worksheet.write(row, 10, item.price)?;
    worksheet.write(row, 11, item.currency.clone())?;
    worksheet.write(row, 12, item.hearts)?;

    Ok(())
}
