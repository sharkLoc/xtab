use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, WriterBuilder};
use log::*;
use regex::RegexBuilder;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn search_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    case: bool,
    invert: bool,
    pat: &str,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {
    let re = RegexBuilder::new(pat).case_insensitive(case).build()?;

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

    let mut flag = false;
    for rec in csv_reader.records().flatten() {
        for each in rec.iter() {
            if re.is_match(each) {
                flag = true;
                continue;
            }
        }
        if flag {
            if invert {
                flag = false;
                continue;
            } else {
                csv_writer.write_record(&rec)?;
                flag = false;
            }
        } else if invert {
            csv_writer.write_record(&rec)?;
        }
    }
    csv_writer.flush()?;

    Ok(())
}
