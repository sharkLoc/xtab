use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use log::*;
use std::{path::PathBuf, time::Instant};

#[allow(clippy::too_many_arguments)]
pub fn slice_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    skip: usize,
    take: usize,
    prefix_num: bool,
    raw_order: bool,
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

    let mut flag = 0usize;
    if prefix_num {
        flag += 1;
    }
    if raw_order {
        flag += 1;
    }
    if flag > 1 {
        error!("only one of the flags --num or --raw is allowed");
        std::process::exit(1);
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
    let mut preid = 0usize;
    if raw_order {
        preid += skip;
    }
    for rec in csv_reader.records().skip(skip).take(take).flatten() {
        if prefix_num || raw_order {
            preid += 1;
            rec_new.push_field(&preid.to_string());
            for each in rec.iter() {
                rec_new.push_field(each);
            }
            csv_writer.write_record(&rec_new)?;
            rec_new.clear();
        } else {
            csv_writer.write_record(&rec)?;
        }
    }
    csv_writer.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
