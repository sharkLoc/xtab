use crate::utils::*;
use anyhow::{Error, Ok};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};
use csv::ReaderBuilder;
use log::*;
use std::{path::PathBuf, time::Instant};

pub fn pretty_csv(
    no_header: bool,
    delimiter: u8,
    table_width: Option<u16>,
    cell_height: Option<usize>,
    alignment: &str,
    header: bool,
    csv: Option<PathBuf>,
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

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);

    // set whole table width
    if let Some(t) = table_width {
        table.set_width(t);
    } else {
        table.width();
    }

    let mut n = 0usize;
    for rec in csv_reader.records().flatten() {
        n += 1;
        let mut row = Row::new();
        // set cell max height
        if let Some(h) = cell_height {
            row.max_height(h);
        }

        for each in rec.iter() {
            let cell = match alignment {
                "left" => Cell::new(each).set_alignment(CellAlignment::Left),
                "center" => Cell::new(each).set_alignment(CellAlignment::Center),
                "right" => Cell::new(each).set_alignment(CellAlignment::Right),
                _ => Cell::new(each),
            };
            row.add_cell(cell);
        }
        //csv has header
        if header && n == 1 {
            table.set_header(row);
            continue;
        }
        table.add_row(row);
    }
    println!("{}", table);

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
