use anyhow::{Ok, Error};
use clap::Parser;

mod loger;
mod args;
mod command;
mod utils;

use command::{
    addheader::addheader_csv, dim::dim_csv, drop::drop_csv, freq::freq_csv, head::head_csv, pretty::pretty_csv, tail::tail_csv, uniq::uniq_csv, view::view_csv
};



fn main() -> Result<(), Error>{

    let cmd = args::Args::parse();
    loger::logger(cmd.verbose, cmd.logfile, cmd.quiet)?;

    match cmd.cmd {
        args::Cmd::view {skip, truncate, output} => {
            view_csv( cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, skip, truncate, cmd.input, output, cmd.compression_level)?;
        }
        args::Cmd::addheader { new_header, output } => {
            addheader_csv(new_header, cmd.delimiter as u8, cmd.out_delimite as u8, cmd.input, output, cmd.compression_level)?;
        }
        args::Cmd::dim {output } => {
            dim_csv(cmd.no_header, cmd.delimiter as u8, cmd.input, output, cmd.compression_level)?;
        }
        args::Cmd::head { num, output } => {
            head_csv(cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, num, cmd.input, output, cmd.compression_level)?;
        }
        args::Cmd::uniq { key , output} => { 
            uniq_csv(cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, key, cmd.input, output, cmd.compression_level)?;
        }
        args::Cmd::tail { num, rev, output } => {
            tail_csv(cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, num, rev, cmd.input, output, cmd.compression_level)?;
        }
        args::Cmd::pretty { width_table, cell_height, aln, header } => {
            pretty_csv(cmd.no_header, cmd.delimiter as u8, width_table, cell_height, &aln, header, cmd.input)?;
        }
        args::Cmd::drop { col_index, invert, output } => {
            drop_csv(cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, col_index, invert, cmd.input, output, cmd.compression_level)?;
        }
        args::Cmd::freq { col_index,key, value, rev, output } => {
            freq_csv(cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, col_index, key, value, rev, cmd.input, output, cmd.compression_level)?;
        }
    }

    Ok(())
}
