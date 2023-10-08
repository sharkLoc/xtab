use anyhow::{Ok, Error};
use clap::Parser;

mod loger;
mod args;
mod command;
use crate::command::view::read_csv;
use crate::command::addheader::addheader_csv;
use crate::command::head::head_csv;



fn main() -> Result<(), Error>{

    loger::logs();
    let cmd = args::Args::parse();

    match cmd.cmd {
        args::Cmd::view { input, output, no_header,  delimiter } => {
            read_csv(no_header, delimiter, input, output, cmd.quiet, )?;
        }
        args::Cmd::addheader { input, output, new_header, delimiter } => {
            addheader_csv(new_header, delimiter, input, output, cmd.quiet)?;
        }
        args::Cmd::head { input, output, no_header, num } => {
            head_csv(no_header, num, input, output, cmd.quiet)?;
        }
    }

    Ok(())
}
