use anyhow::{Error, Ok};
use args::VERSION;
use clap::Parser;
use log::info;

mod args;
mod command;
mod error;
mod loger;
mod utils;

use command::{
    addheader::addheader_csv, csv2xlsx::csv_xlsx, dim::dim_csv, drop::drop_csv,
    flatten::flatten_csv, freq::freq_csv, head::head_csv, pretty::pretty_csv, replace::replace_csv,
    reverse::reverse_csv, sample::sample_csv, search::search_csv, slice::slice_csv, tail::tail_csv,
    transpose::transpose_csv, uniq::uniq_csv, view::view_csv, xlsx2csv::xlsx_csv,
};

fn main() -> Result<(), Error> {
    let cmd = args::Args::parse();
    loger::logger(cmd.verbose, cmd.logfile, cmd.quiet)?;
    info!("xtab version: {}", VERSION);
    let start = std::time::Instant::now();

    match cmd.cmd {
        args::Cmd::view {
            skip,
            truncate,
            output,
        } => {
            view_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                skip,
                truncate,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::addheader { new_header, output } => {
            addheader_csv(
                new_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::dim { output } => {
            dim_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::head { num, output } => {
            head_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                num,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::uniq { key, output } => {
            uniq_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                key,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::tail { num, rev, output } => {
            tail_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                num,
                rev,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::pretty {
            width_table,
            cell_height,
            aln,
            header,
        } => {
            pretty_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                width_table,
                cell_height,
                &aln,
                header,
                cmd.input,
            )?;
        }
        args::Cmd::drop {
            col_index,
            invert,
            output,
        } => {
            drop_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                col_index,
                invert,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::freq {
            col_index,
            key,
            value,
            rev,
            output,
        } => {
            freq_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                col_index,
                key,
                value,
                rev,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::csv2xlsx { bold, border, xlsx } => {
            csv_xlsx(
                cmd.no_header,
                cmd.delimiter as u8,
                bold,
                border,
                cmd.input,
                &xlsx,
            )?;
        }
        args::Cmd::xlsx2csv { xlsx, idx, csv } => {
            xlsx_csv(
                xlsx,
                idx,
                cmd.out_delimite as u8,
                csv,
                cmd.compression_level,
            )?;
        }
        args::Cmd::flatten { separator, output } => {
            flatten_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                separator,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::slice {
            skip,
            len,
            pre_num,
            raw_order,
            output,
        } => {
            slice_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                skip,
                len,
                pre_num,
                raw_order,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::reverse { output } => {
            reverse_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::sample { num, seed, output } => {
            sample_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                num,
                seed,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::search {
            pat,
            case,
            invert,
            output,
        } => {
            search_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                case,
                invert,
                &pat,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::transpose { output } => {
            transpose_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
        args::Cmd::replace {
            col_index,
            src,
            dst,
            all,
            output,
        } => {
            replace_csv(
                cmd.no_header,
                cmd.delimiter as u8,
                cmd.out_delimite as u8,
                &col_index,
                &src,
                &dst,
                all,
                cmd.input,
                output,
                cmd.compression_level,
            )?;
        }
    }

    info!("time elapsed is: {:?}", start.elapsed());

    Ok(())
}
