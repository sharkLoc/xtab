use anyhow::{Ok, Error};
use clap::Parser;

mod loger;
mod args;
mod command;
mod utils;

use command::{
    addheader::*, head::*, view::*,
};



fn main() -> Result<(), Error>{

    let cmd = args::Args::parse();
    loger::logger(cmd.verbose, cmd.logfile, cmd.quiet)?;

    match cmd.cmd {
        args::Cmd::view {skip} => {
            view_csv( cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, skip, cmd.input, cmd.output, cmd.compression_level)?;
        }
        args::Cmd::addheader { new_header } => {
            addheader_csv(new_header, cmd.delimiter as u8, cmd.out_delimite as u8, cmd.input, cmd.output)?;
        }
        args::Cmd::head { num } => {
            head_csv(cmd.no_header, cmd.delimiter as u8, cmd.out_delimite as u8, num, cmd.input, cmd.output)?;
        }
    }

    Ok(())
}
