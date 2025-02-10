use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder,WriterBuilder,StringRecord};
use log::*;
use std::path::PathBuf;

pub fn dim_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    tabin: bool,
    tabout: bool,
    csv: Option<PathBuf>,
    csvo: Option<PathBuf>,
    compression_level: u32,
) -> Result<(), Error> {

    let mut csv_reader = if tabin {
        ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter('\t' as u8)
        .from_reader(file_reader(csv.as_ref())?)
    } else {
        ReaderBuilder::new()
        .has_headers(no_header)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?)
    };

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

    let mut csv_writer =  if tabout {
        WriterBuilder::new()
        .delimiter('\t' as u8)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?)
    } else {
        WriterBuilder::new()
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?)
    };

    let mut ctx = StringRecord::new();
    ctx.push_field("file");
    ctx.push_field("rows");
    ctx.push_field("cols");
    csv_writer.write_record(&ctx)?;
    ctx.clear();

    if let Some(filename) = csv {
        ctx.push_field(filename.to_str().unwrap());
    } else {
        ctx.push_field("-");
    }
    ctx.push_field(format!("{}",row).as_str());
    ctx.push_field(format!("{}",col.unwrap()).as_str());
    csv_writer.write_record(&ctx)?;
    csv_writer.flush()?;

    Ok(())
}
