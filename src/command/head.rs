use csv::{WriterBuilder, ReaderBuilder};
use anyhow::{Ok, Error};
use std::{path::PathBuf, time::Instant};
use log::*;
use crate::utils::*;


pub fn head_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    num: usize,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {
    let start = Instant::now();

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?);
    
    match csv {
        Some(csv) => info!("read file from: {:?}",csv),
        None => info!("read file from stdin ")
    }

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?);

    for rec in csv_reader.records().flatten().take(num) {
        csv_writer.write_record(&rec)?;
    }
    csv_writer.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}