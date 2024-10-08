use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use log::*;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn replace_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    index_str: &str,
    src: &str,
    dst: &str,
    all: bool,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?);

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

    match csv {
        Some(csv) => info!("read file from: {}", csv.display()),
        None => info!("read file from stdin "),
    }

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?);

    let mut rec_new = StringRecord::new();
    let mut count = 0usize;
    for rec in csv_reader.records().flatten() {
        for (idx, each) in rec.iter().enumerate() {
            if all || col_index.contains(&(idx + 1)) {
                if each == src {
                    rec_new.push_field(dst);
                    count += 1;
                } else {
                    rec_new.push_field(each);
                }
            } else {
                rec_new.push_field(each);
            }
        }
        csv_writer.write_record(&rec_new)?;
        rec_new.clear();
    }
    csv_writer.flush()?;

    info!("total replace cell count: {}", count);
    Ok(())
}
