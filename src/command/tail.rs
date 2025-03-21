use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, WriterBuilder};
use log::*;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn tail_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    tabin: bool,
    tabout: bool,
    num: usize,
    rev: bool,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {

    let mut csv_reader = if tabin {
        ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(file_reader(csv.as_ref())?)
    } else {
        ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?)
    };

    match csv {
        Some(csv) => info!("read file from: {}", csv.display()),
        None => info!("read file from stdin "),
    }
    let mut recs = vec![];
    for rec in csv_reader.records().flatten() {
        recs.push(rec);
    }

    let mut csv_writer = if tabout {
        WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(b'\t')
        .from_writer(file_writer(csvo.as_ref(), compression_level)?)
    } else {
        WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?)
    }; 

    if rev {
        info!("output reversed result");
        for rec in recs.iter().rev().take(num) {
            csv_writer.write_record(rec)?;
        }
    } else {
        for rec in recs.iter().rev().take(num).rev() {
            csv_writer.write_record(rec)?;
        }
    }

    csv_writer.flush()?;

    Ok(())
}
