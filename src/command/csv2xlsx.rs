use crate::utils::*;
use anyhow::{Error, Ok};
use csv::ReaderBuilder;
use log::info;
use rust_xlsxwriter::{Format, FormatBorder, Workbook};
use std::{path::PathBuf, time::Instant};

pub fn csv_xlsx(
    no_header: bool,
    delimiter: u8,
    bold_first: bool,
    border_first: bool,
    csv: Option<PathBuf>,
    xlsx: &str,
) -> Result<(), Error> {
    let start = Instant::now();

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?);

    match csv {
        Some(csv) => info!("read file from: {:?}", csv),
        None => info!("read file from stdin "),
    }

    let mut work_book = Workbook::new();
    let work_sheet = work_book.add_worksheet();

    let format = match (bold_first, border_first) {
        (true, true) => Format::new().set_bold().set_border(FormatBorder::Double),
        (false, false) => Format::new(),
        (true, false) => Format::new().set_bold(),
        (false, true) => Format::new().set_border(FormatBorder::Double),
    };

    for (nrow, rec) in csv_reader.records().flatten().enumerate() {
        for (ncol, cell) in rec.iter().enumerate() {
            if (bold_first || border_first) && nrow == 0 {
                work_sheet.write_with_format(nrow as u32, ncol as u16, cell, &format)?;
            } else {
                work_sheet.write(nrow as u32, ncol as u16, cell)?;
            }
        }
    }
    work_book.save(xlsx)?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
