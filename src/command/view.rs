use csv::{WriterBuilder, ReaderBuilder};
use anyhow::{Ok, Error};
use std::path::PathBuf;
use log::info;
use crate::command::utils::*;


pub fn read_csv(
    no_header: bool,
    delimiter: String,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    quiet: bool,
) -> Result<(), Error> {
    if !quiet {
        info!("reading from: {:?}",csv.clone().unwrap());
    }

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(no_header)
        .delimiter(delimiter.as_bytes()[0])
        .from_reader(file_reader(&csv)?);

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(delimiter.as_bytes()[0])
        .from_writer(file_writer(&csvo)?);

    for rec in csv_reader.records().flatten() {
        csv_writer.write_record(&rec)?;
    }

    Ok(())
}