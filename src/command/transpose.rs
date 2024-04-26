use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use log::*;
use std::{collections::HashMap, path::PathBuf, time::Instant};


pub fn transpose_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
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
        Some(csv) => info!("read file from: {:?}", csv),
        None => info!("read file from stdin "),
    }

    let mut df_hash: HashMap<usize, Vec<String>> = HashMap::new();
    for rec in csv_reader.records().flatten() {
        for (col, each) in rec.iter().enumerate() {
            df_hash.entry(col).or_default().push(each.to_string());
        }
    }

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?);

    let mut str_rec = StringRecord::new();
    for i in 0..df_hash.len() {
        let vec = df_hash.get(&i).unwrap();
        for v in vec.iter() {
            str_rec.push_field(v);
        }
        csv_writer.write_record(&str_rec)?;
        str_rec.clear();
    }
    csv_writer.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
