use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use log::info;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn addheader_csv(
    new_header: String,
    delimiter: u8,
    out_delimiter: u8,
    tabin: bool,
    tabout: bool,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {

    let mut csv_reader =  if tabin {
        ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .has_headers(false)
        .from_reader(file_reader(csv.as_ref())?)
    } else {
        ReaderBuilder::new()
        .delimiter(delimiter)
        .flexible(true)
        .has_headers(false)
        .from_reader(file_reader(csv.as_ref())?)
    };

    match csv {
        Some(csv) => info!("read file from: {}", csv.display()),
        None => info!("read file from stdin "),
    }
    info!("new header is: {}", new_header);

    let mut csv_writer =  if tabout {
        WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(file_writer(csvo.as_ref(), compression_level)?)
    } else {
        WriterBuilder::new()
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?)
    };

    // new header
    let new = new_header.split(',').collect::<Vec<&str>>();
    csv_reader.set_headers(StringRecord::from(new));

    for rec in csv_reader.records().flatten() {
        csv_writer.write_record(&rec)?;
    }
    csv_writer.flush()?;

    Ok(())
}
