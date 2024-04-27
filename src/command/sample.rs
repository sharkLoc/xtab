use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, WriterBuilder};
use log::*;
use rand::{prelude::*, Rng};
use rand_pcg::Pcg64;
use std::{path::PathBuf, time::Instant};

pub fn sample_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    num: usize,
    seed: u64,
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
        Some(csv) => info!("read file from: {}", csv.display()),
        None => info!("read file from stdin "),
    }
    info!("rand seed is: {}", seed);

    // init rand seed
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut get_rec = vec![];
    for (idx, rec) in csv_reader.records().flatten().enumerate() {
        if idx < num {
            get_rec.push(rec);
        } else {
            let ret = rng.gen_range(0..=idx);
            if ret < num {
                get_rec[ret] = rec;
            }
        }
    }

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?);

    for rec in get_rec.iter() {
        csv_writer.write_record(rec)?;
    }
    csv_writer.flush()?;
    info!("time elapsed is: {:?}", start.elapsed());

    Ok(())
}
