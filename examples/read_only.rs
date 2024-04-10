use edit_xlsx::{Workbook, WorkbookResult, Read, Write, Col, Row, Format, FormatColor};

fn main() -> WorkbookResult<()> {
    // from an existed workbook
    let mut reading_book = Workbook::from_path("C:\\Users\\00331594\\Desktop\\bg_test.xlsx")?;
    reading_book.finish();
    // Read the first sheet
    let reading_sheet = reading_book.read_worksheet(1)?;
    let mut writing_book = Workbook::new();
    let writing_sheet = writing_book.get_worksheet(1)?;
    writing_sheet.set_default_row(writing_sheet.get_default_row());
    let bg_format = reading_sheet.read_format();
    writing_sheet.set_column_with_format((1, 1, 1, 16384), 20.0, &bg_format)?;
    for row in 1..=reading_sheet.max_row() {
        for col in 1..=reading_sheet.max_column() {
            let text = reading_sheet.read((row, col)).unwrap_or_default();
            let format = reading_sheet.read_format((row, col)).unwrap_or_default();
            writing_sheet.write_with_format((row, col), text, &format).unwrap();
            if let Ok(height) = writing_sheet.get_row(row) {
                writing_sheet.set_row(row, height)?;
            }
        }
    }
    writing_book.save_as("./examples/new.xlsx")?;
    Ok(())
}