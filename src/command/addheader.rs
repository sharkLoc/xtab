use csv::{WriterBuilder, ReaderBuilder,StringRecord};
use anyhow::{Ok, Error};
use std::path::PathBuf;
use log::info;
use crate::command::utils::*;


pub fn addheader_csv(
    new_header: String,
    delimiter: String,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    quiet: bool,
) -> Result<(), Error> {
    if !quiet {
        info!("reading from: {:?}",csv);
        info!("new header is: {}",new_header);
    }
    
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .has_headers(false)
        .from_reader(file_reader(&csv)?);

    let mut csv_writer = WriterBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .from_writer(file_writer(&csvo)?);
    
    // new header
    let new = new_header.split(",").collect::<Vec<&str>>();
    csv_reader.set_headers(StringRecord::from(new));

    for rec in csv_reader.records().flatten() {
        csv_writer.write_record(&rec)?;
    }

    Ok(())
}