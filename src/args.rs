use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "size_t",
    version = "version 0.0.1",
    next_line_help = false,
    about = "CSV command line utilities",
    long_about = None,
)]
pub struct Args {
    /// be quiet and do not show extra information
    #[arg(short = 'q', long = "quiet", global(true))]
    pub quiet: bool,

    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Parser)]
#[allow(non_camel_case_types)]
pub enum Cmd {
    /// Show csv content
    view {
        /// Input csv file name[.gz], or read from stdin
        input: Option<PathBuf>,
        /// output csv file name[.gz] or write to stdout
        output: Option<PathBuf>,
        /// If set, the original header row excluded from output
        #[arg(short = 'n', long = "no-header")]
        no_header: bool,
        /// set delimiter
        #[arg(short = 'd', long = "delimiter", default_value_t = String::from(","))]
        delimiter: String,
    },
    /// Set new header for CSV file
    addheader {
        /// Input csv file name[.gz], or read from stdin
        input: Option<PathBuf>,
        /// output csv file name[.gz] or write to stdout
        output: Option<PathBuf>,
        /// set new header, e.g -N colum1,column2...
        #[arg(short = 'N', long = "new-header")]
        new_header: String,
        /// set delimiter
        #[arg(short = 'd', long = "delimiter", default_value_t = String::from("\t"))]
        delimiter: String,
    },
    /// Get first N records from CSV file
    head {
        /// Input csv file name[.gz], or read from stdin
        input: Option<PathBuf>,
        /// output csv file name[.gz] or write to stdout
        output: Option<PathBuf>,
        /// If set, the original header row excluded from output
        #[arg(short = 'n', long = "no-header")]
        no_header: bool,
        /// print first N records
        #[arg(short = 'N', long = "num", default_value_t = 10)]
        num: usize,
    },
}