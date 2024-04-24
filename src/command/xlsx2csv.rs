use crate::utils::*;
use anyhow::{Error, Ok};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use csv::{StringRecord, WriterBuilder};
use log::*;
use std::{path::PathBuf, time::Instant};

pub fn xlsx_csv(
    xlsx: Option<PathBuf>,
    sheet_idx: usize,
    out_delimiter: u8,
    csv: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {
    let start = Instant::now();

    match &xlsx {
        Some(x) => info!("read file from: {:?}", x),
        None => {
            error!("xlsx file not provided");
            std::process::exit(1);
        }
    }
    if sheet_idx == 0 {
        error!("sheet index start from 1");
        std::process::exit(1);
    }

    let mut xlsx: Xlsx<_> = open_workbook(xlsx.unwrap())?;
    let names = xlsx.sheet_names();
    info!("xlsx file contains {} sheets: {:?}", names.len(), names);
    info!("select sheet: \"{}\"", names[sheet_idx - 1]);

    let sheet = xlsx.worksheet_range_at(sheet_idx - 1);
    let mut csv_writer = WriterBuilder::new()
        .has_headers(true)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csv.as_ref(), compression_level)?);

    if let Some(tab) = sheet {
        let df = tab?;
        for row in df.rows() {
            let mut rec = StringRecord::new();
            for col in row.iter() {
                if let Some(v) = col.as_string() {
                    rec.push_field(&v);
                } else {
                    rec.push_field("")
                }
            }
            csv_writer.write_record(&rec)?;
        }
    }
    csv_writer.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
