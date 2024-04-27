use crate::utils::*;
use anyhow::{Error, Ok};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use log::*;
use std::{collections::HashMap, path::PathBuf, time::Instant};

pub fn freq_csv(
    no_header: bool,
    delimiter: u8,
    out_delimiter: u8,
    index_str: String,
    sort_key: bool,
    sort_value: bool,
    sort_reverse: bool,
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
    if sort_key {
        flag += 1;
    }
    if sort_value {
        flag += 1;
    }
    if flag > 1 {
        error!("only one of the flags --sort-by-key, --sort-by-freq is allowed");
        std::process::exit(1);
    }
    match csv {
        Some(csv) => info!("read file from: {}", csv.display()),
        None => info!("read file from stdin "),
    }

    let mut col_index = vec![];
    for idx in index_str.split(',').collect::<Vec<&str>>() {
        let idx = idx.parse::<usize>()?;
        col_index.push(idx);
        if idx == 0 {
            error!("col_index error : {}, start from 1", idx);
            std::process::exit(1);
        }
    }

    let mut hash: HashMap<String, usize> = HashMap::new();
    let mut raw_order = vec![];
    for rec in csv_reader.records().flatten() {
        let mut keys = vec![];
        for (idx, each) in rec.iter().enumerate() {
            if col_index.contains(&(idx + 1)) {
                keys.push(each);
                keys.push(",");
            }
        }
        let key = keys.concat();
        if !raw_order.contains(&key) {
            raw_order.push(key.clone());
        }
        *hash.entry(key).or_insert(0) += 1;
    }

    let mut csv_writer = WriterBuilder::new()
        .has_headers(no_header)
        .delimiter(out_delimiter)
        .from_writer(file_writer(csvo.as_ref(), compression_level)?);

    if sort_key {
        let mut count = hash.iter().collect::<Vec<(&String, &usize)>>();
        if sort_reverse {
            count.sort_by(|x, y| y.0.cmp(x.0));
        } else {
            count.sort_by(|x, y| x.0.cmp(y.0));
        }

        let mut rec_new = StringRecord::new();
        for (k, v) in count {
            let mut tmp_keys = k.split(',').collect::<Vec<&str>>();
            tmp_keys.retain(|&x| x != ""); // strip last "" in tmp_keys

            for each in tmp_keys {
                rec_new.push_field(each);
            }
            rec_new.push_field(&v.to_string());
            csv_writer.write_record(&rec_new)?;
            rec_new.clear();
        }
    } else if sort_value {
        let mut count = hash.iter().collect::<Vec<(&String, &usize)>>();
        if sort_reverse {
            count.sort_by(|x, y| y.1.cmp(x.1));
        } else {
            count.sort_by(|x, y| x.1.cmp(y.1));
        }

        let mut rec_new = StringRecord::new();
        for (k, v) in count {
            let mut tmp_keys = k.split(',').collect::<Vec<&str>>();
            tmp_keys.retain(|&x| x != ""); // strip last "" in tmp_keys

            for each in tmp_keys {
                rec_new.push_field(each);
            }
            rec_new.push_field(&v.to_string());
            csv_writer.write_record(&rec_new)?;
            rec_new.clear();
        }
    } else {
        let mut rec_new = StringRecord::new();
        for k in raw_order.iter() {
            let mut tmp_keys = k.split(',').collect::<Vec<&str>>();
            tmp_keys.retain(|&x| x != ""); // strip last "" in tmp_keys

            for each in tmp_keys {
                rec_new.push_field(each);
            }
            let v = hash.get(k).unwrap();
            rec_new.push_field(&v.to_string());
            csv_writer.write_record(&rec_new)?;
            rec_new.clear();
        }
    }
    csv_writer.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
