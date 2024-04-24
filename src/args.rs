use std::path::PathBuf;
use clap::{value_parser, Parser};


#[derive(Debug, Parser)]
#[command(
    name = "xtab",
    author = "sharkLoc",
    version = "0.0.3",
    next_line_help = false,
    about = "CSV command line utilities",
    long_about = "A simple and cross-platform program for CSV file manipulation"
)]
#[command(propagate_version = false, disable_help_flag = false, disable_version_flag = true)]
#[command(before_help=r"xtab supports reading and writing gzip/bzip2/xz format file.
Compression level:
  format   range   default   crate
  gzip     1-9     6         https://crates.io/crates/flate2
  bzip2    1-9     6         https://crates.io/crates/bzip2
  xz       1-9     6         https://crates.io/crates/xz2"
)]
#[command(help_template = "{name} -- {about}\n\nVersion: {version}\
    \nAuthors: {author} <mmtinfo@163.com>\
    \nSource code: https://github.com/sharkLoc/xtab.git\
    \n\n{before-help}
{usage-heading} {usage}\n\n{all-args}\n\nUse \"xtab help [command]\" for more information about a command")]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Cmd,

    /// Input csv file name, if file not specified read data from stdin
    #[arg(value_name = "CSV", global = true, help_heading = Some("Global Arguments"))]
    pub input: Option<PathBuf>,

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
    /// Set new header for CSV file
    #[command(visible_alias = "ah")]
    addheader {
        /// Set new header, e.g -n "colum1,column2..."
        #[arg(short = 'n', long = "new-header", value_name = "STR")]
        new_header: String,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Convert CSV/TSV files to XLSX file
    #[command(visible_alias = "c2x")]
    csv2xlsx {
        /// If set, bold first line in XLSX file
        #[arg(short = 'B', long = "bold-first", help_heading = Some("FLAGS"))]
        bold: bool,
        /// If set, border first line in XLSX file
        #[arg(short = 'b', long = "border-first", help_heading = Some("FLAGS"))]
        border: bool,
        /// Output xlsx file name, e.g, result.xlsx
        #[arg(short = 'x', long = "xlsx", value_name = "FILE", default_value_t = String::from("Sheet1.xlsx"))]
        xlsx: String,
    },
    /// Dimensions of CSV file
    dim {
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Drop or Select CSV fields by columns index
    drop {
        /// Select columns index, e.g -c 2,3,5
        #[arg(short = 'c', long = "col-index", value_name = "STR")]
        col_index: String,
        /// invert the sense of matching, to select non-matching fields
        #[arg(short = 'u', long = "invert-match", help_heading = Some("FLAGS"))]
        invert: bool,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// freq
    freq {
        /// Select columns index, e.g -c 2,3,5
        #[arg(short = 'c', long = "col-index", value_name = "STR")]
        col_index: String,
        /// Sort by key
        #[arg(short = 'k', long = "sort-by-key", help_heading = Some("FLAGS"))]
        key: bool,
        /// sort by frequency
        #[arg(short = 'n', long = "sort-by-freq", help_heading = Some("FLAGS"))]
        value: bool,
        /// Output reversed result
        #[arg(short = 'r', long = "rev", help_heading = Some("FLAGS"))]
        rev: bool,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Print first N records from CSV file
    head {
        /// Print first N records, if option "--no-header" enabled, the original header row excluded from output
        #[arg(short = 'n', long = "num", default_value_t = 10, value_name = "INT")]
        num: usize,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Convert CSV to a readable aligned table
    pretty {
        /// Set the whole table width
        #[arg(short = 'w', long = "width-table", value_name = "INT", value_parser = value_parser!(u16).range(0..=65535))]
        width_table: Option<u16>,
        /// If set, truncate content of cells which occupies more than INT lines of space
        #[arg(short = 't', long="truncate", value_name = "INT")]
        cell_height: Option<usize>,
        /// Set the alignment of content for each cell, possible values: {left, center, right}
        #[arg(short ='a', long = "aln", value_name = "STR", default_value_t = String::from("left"))]
        aln: String,
        /// Show header in different style
        #[arg(long = "header", help_heading = Some("FLAGS"))]
        header: bool,
    },

    /// Print last N records from CSV file
    tail {
        /// Print last N records, if option "--no-header" enabled, the original header row excluded from output
        #[arg(short = 'n', long = "num", default_value_t = 10, value_name = "INT")]
        num: usize,
        /// Output reversed result
        #[arg(short = 'r', long = "reverse", help_heading = Some("FLAGS"))]
        rev: bool,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Unique data with keys
    uniq {
        /// Select these fields as keys. e.g -k 2,3,5
        #[arg(short = 'k', long = "key", value_name = "STR")]
        key: String,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Convert XLSX to CSV format
    #[command(visible_alias = "x2c")]
    xlsx2csv {
        /// Input XLSX file
        #[arg(value_name = "FILE")]
        xlsx: Option<PathBuf>,
        /// Select Nth sheet to retrieve
        #[arg(short = 'i', long = "sheet-index", value_name = "INT", default_value_t=1)]
        idx: usize,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out-csv", value_name = "FILE")]
        csv: Option<PathBuf>,
    },

    /// Show CSV file content
    view {
        /// Skip first N records, not include the header row when option "--no-header" enabled. eg "-s 10 --no-header" will skip 11 records
        #[arg(short = 's', long = "skip", value_name = "INT", default_value_t = 0 )]
        skip: usize,
        /// If enabled, truncate each record to N fields, if N is greater than the number of fields in this record, then this has no effect
        #[arg(short = 't', long = "truncate", value_name = "INT")]
        truncate: Option<usize>,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },
}