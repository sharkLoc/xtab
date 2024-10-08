use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, WriterBuilder};
use log::*;
use std::path::PathBuf;

pub fn uniq_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    index_str: String,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?);

    match csv {
        Some(csv) => info!("read file from: {}", csv.display()),
        None => info!("read file from stdin "),
    }

    let mut col_index = vec![];
    for idx in index_str.split(',').collect::<Vec<&str>>() {
        let idx = idx.parse::<usize>()?;
        if col_index.contains(&idx) {
            warn!("duplicate columns index {}, keep first one", idx);
            continue;
        } else {
            col_index.push(idx);
        }
        if idx == 0 {
            error!("col_index error : {}, start from 1", idx);
            std::process::exit(1);
        }
    }

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?);

    let mut row = 0usize;
    let mut keys = vec![];
    for rec in csv_reader.records().flatten() {
        row += 1;
        let mut cols = vec![];

        for idx in col_index.iter() {
            match rec.get(idx - 1) {
                Some(x) => cols.push(x),
                None => {
                    error!("record on line {}: wrong index of fields", row);
                    std::process::exit(1);
                }
            }
        }

        let key = cols.concat();
        if keys.contains(&key) {
            continue;
        } else {
            keys.push(key);
            csv_writer.write_record(&rec)?;
        }
    }
    csv_writer.flush()?;

    Ok(())
}
