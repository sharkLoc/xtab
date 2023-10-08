use csv::{WriterBuilder, ReaderBuilder};
use anyhow::{Ok, Error};
use std::path::PathBuf;
use log::info;
use crate::command::utils::*;


pub fn head_csv(
    no_header: bool,
    //delimiter: String,
    num: usize,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    quiet: bool,
) -> Result<(), Error> {
    if !quiet {
        if let Some(csv) = csv.clone() {
            info!("reading from: {:?}",csv);
        } else {
            info!("reading from: stdin");
        }
        info!("get top {} records", num);
    }

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(no_header)
        //.delimiter(delimiter.as_bytes()[0])
        .from_reader(file_reader(&csv)?);

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        //.delimiter(delimiter.as_bytes()[0])
        .from_writer(file_writer(&csvo)?);

    for rec in csv_reader.records().flatten().take(num) {
        csv_writer.write_record(&rec)?;
    }

    Ok(())
}