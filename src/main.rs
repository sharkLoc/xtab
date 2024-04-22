use anyhow::{Ok, Error};
use clap::Parser;

mod loger;
mod args;
mod command;
mod utils;

use command::{
    addheader::*, dim::dim_csv, head::*, view::*
};



fn main() -> Result<(), Error>{

    let cmd = args::Args::parse();
    loger::logger(cmd.verbose, cmd.logfile, cmd.quiet)?;

    match cmd.cmd {
        args::Cmd::view {skip, truncate} => {
            view_csv( cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, skip, truncate, cmd.input, cmd.output, cmd.compression_level)?;
        }
        args::Cmd::addheader { new_header } => {
            addheader_csv(new_header, cmd.delimiter as u8, cmd.out_delimite as u8, cmd.input, cmd.output, cmd.compression_level)?;
        }
        args::Cmd::dim {  } => {
            dim_csv(cmd.no_header, cmd.delimiter as u8, cmd.input, cmd.output, cmd.compression_level)?;
        }
        args::Cmd::head { num } => {
            head_csv(cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, num, cmd.input, cmd.output, cmd.compression_level)?;
        }
        args::Cmd::uniq {  } => { todo!()}
    }

    Ok(())
}
