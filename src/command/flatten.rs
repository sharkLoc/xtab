use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use log::*;
use std::path::PathBuf;

pub fn flatten_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    sep: Option<char>,
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

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?);

    let mut header = vec![];
    let mut rec_new = StringRecord::new();
    for (row, rec) in csv_reader.records().flatten().enumerate() {
        if row == 0 {
            for each in rec.iter() {
                header.push(each.to_string());
            }
        } else {
            for (head, txt) in header.iter().zip(rec.iter()) {
                rec_new.push_field(head);
                rec_new.push_field(txt);
                csv_writer.write_record(&rec_new)?;
                rec_new.clear();
            }
            if let Some(sep) = sep {
                rec_new.push_field(&sep.to_string());
                rec_new.push_field(&sep.to_string());
                csv_writer.write_record(&rec_new)?;
                rec_new.clear();
            }
        }
    }
    csv_writer.flush()?;

    Ok(())
}
