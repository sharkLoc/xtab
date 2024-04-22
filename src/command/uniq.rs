use csv::{WriterBuilder, ReaderBuilder};
use anyhow::{Ok, Error};
use std::{path::PathBuf, time::Instant};
use log::*;
use crate::utils::*;


pub fn uniq_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {
    let start = Instant::now();

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}