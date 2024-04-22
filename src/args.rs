use std::path::PathBuf;
use clap::{value_parser, Parser};


#[derive(Debug, Parser)]
#[command(
    name = "xtab",
    author = "sharkLoc",
    version = "0.0.2",
    next_line_help = false,
    about = "CSV command line utilities"
)]
#[command(long_about = "A simple and cross-platform program for CSV file manipulation")]
#[command(before_help=r"xtab supports reading and writing gzip/bzip2/xz format file.
Compression level:
  format   range   default   crate
  gzip     1-9     6         https://crates.io/crates/flate2
  bzip2    1-9     6         https://crates.io/crates/bzip2
  xz       1-9     6         https://crates.io/crates/xz2"
)]
#[command(help_template = "{name} -- {about}\nVersion: {version}\
    \n\nAuthors: {author} <mmtinfo@163.com>\
    \nSource code: https://github.com/sharkLoc/xtab.git\
    \n\n{before-help}
{usage-heading} {usage}\n\n{all-args}\n\nUse \"xtab help [command]\" for more information about a command")]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Cmd,

    /// Input csv file name, if file not specified read data from stdin
    #[arg(value_name = "CSV", global = true, help_heading = Some("Global Arguments"))]
    pub input: Option<PathBuf>,

    /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
    #[arg(short = 'o', long = "out", value_name = "FILE", global = true, help_heading = Some("Global Arguments"))]
    pub output: Option<PathBuf>,

    /// If set, the first row is treated as a special header row, and the original header row excluded from output
    #[arg(short = 'H', long = "no-header", global = true, help_heading = Some("Global FLAGS"))]
    pub no_header: bool,

    /// Set delimiter for input csv file, e.g., -d $'\t' for tab
    #[arg(short = 'd', long = "delimiter", default_value_t = ',', global = true, value_name = "CHAR", help_heading = Some("Global Arguments"))]
    pub delimiter: char,
    /// Set delimiter for output CSV file, e.g., -D $'\t' for tab
    #[arg(short = 'D', long = "out-delimite", default_value_t = ',', global = true, value_name = "CHAR", help_heading = Some("Global Arguments"))]
    pub out_delimite: char,

    /// If file name specified, write log message to this file, or write to stderr
    #[arg(long = "log", global = true, help_heading = Some("Global Arguments"), value_name = "FILE")]
    pub logfile: Option<String>,

    /// Set compression level 1 (compress faster) - 9 (compress better) for gzip/bzip2/xz output file, just work with option -o/--out
    #[arg(long = "compress-level", default_value_t = 6, global = true, value_parser = value_parser!(u32).range(1..=9), 
        value_name = "INT", help_heading = Some("Global Arguments")
    )]
    pub compression_level: u32,

    /// Control verbosity of logging, possible values: {error, warn, info, debug, trace}
    #[arg(short = 'v', long = "verbosity", global = true, value_name = "STR",
        default_value_t = String::from("debug"), help_heading = Some("Global Arguments")
    )]
    pub verbose: String,

    /// Be quiet and do not show any extra information
    #[arg(short = 'q', long = "quiet", global= true, help_heading = Some("Global FLAGS"))]
    pub quiet: bool,
}

#[derive(Debug, Parser)]
#[allow(non_camel_case_types)]
pub enum Cmd {
    /// Show CSV file content
    view {
        /// Skip first N records, not include the header row when option "--no-header" enabled. eg "-s 10 --no-header" will skip 11 records
        #[arg(short = 's', long = "skip", value_name = "INT", default_value_t = 0 )]
        skip: usize,
    },
    /// Set new header for CSV file
    #[command(visible_alias = "ah")]
    addheader {
        /// Set new header, e.g -N "colum1,column2..."
        #[arg(short = 'N', long = "new-header", value_name = "STR")]
        new_header: String,
    },
    /// Get first N records from CSV file
    head {
        /// Print first N records, if option "--no-header" enabled, the original header row excluded from output
        #[arg(short = 'n', long = "num", default_value_t = 10, value_name = "INT")]
        num: usize,
    },
}