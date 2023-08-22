use rust_xlsxwriter::{Url, Worksheet};

use crate::models::item_metadata::ItemMetadata;

pub fn write_row(
    item: &ItemMetadata,
    worksheet: &mut Worksheet,
    row: u32,
) -> Result<(), rust_xlsxwriter::XlsxError> {
    worksheet.write(row, 0, item.item.name.name.name.clone())?;
    worksheet.write(row, 1, item.item.name.name.name_translated.clone())?;
    worksheet.write(row, 2, Url::new(item.item.name.url.clone()))?;
    worksheet.write(row, 3, item.author.name.name.clone())?;
    worksheet.write(row, 4, item.author.name.name_translated.clone())?;
    worksheet.write(row, 5, Url::new(item.author.url.clone()))?;
    worksheet.write(row, 6, item.category.category.name.name.clone())?;
    worksheet.write(row, 7, false)?;
    worksheet.write(row, 8, false)?;
    worksheet.write(row, 9, item.price.number)?;
    worksheet.write(row, 10, item.price.unit.clone())?;
    worksheet.write(row, 11, item.hearts)?;

    Ok(())
}
