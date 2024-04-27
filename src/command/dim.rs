use crate::utils::*;
use anyhow::{Error, Ok};
use csv::ReaderBuilder;
use log::*;
use std::{path::PathBuf, time::Instant};

pub fn dim_csv(
    no_header: bool,
    delimiter: u8,
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

    match &csv {
        Some(csv) => info!("read file from: {}", csv.display()),
        None => info!("read file from stdin "),
    }

    let mut row = 0usize;
    let mut col = None::<usize>;
    for rec in csv_reader.records().flatten() {
        row += 1;
        if let Some(col) = col {
            if col != rec.len() {
                error!("record on line {}: wrong number of fields", row);
                std::process::exit(1);
            }
        } else {
            col = Some(rec.len());
        }
    }

    let mut out_writer = file_writer(csvo.as_ref(), compression_level)?;

    let buf = if let Some(file) = csv {
        format!("file\trows\tcols\n{}\t{}\t{}\n", file.display(), row, col.unwrap())
    } else {
        format!("file\trows\tcols\n-\t{}\t{}\n", row, col.unwrap())
    };
    out_writer.write_all(buf.as_bytes())?;
    out_writer.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
