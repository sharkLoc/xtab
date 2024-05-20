use clap::{value_parser, ArgAction, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "xtab",
    author = "sharkLoc",
    version = "0.0.7",
    next_line_help = false,
    about = "CSV command line utilities",
    long_about = "A simple and cross-platform program for CSV file manipulation"
)]
#[command(
    propagate_version = true,
    disable_help_flag = true,
    disable_version_flag = true
)]
#[command(
    before_help = r"xtab supports reading and writing gzip/bzip2/xz format file.
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

    /// Input CSV file name, if file not specified read data from stdin
    #[arg(value_name = "CSV", global = true, help_heading = Some("Global Arguments"))]
    pub input: Option<PathBuf>,

    /// If set, the first row is treated as a special header row, and the original header row excluded from output
    #[arg(short = 'H', long = "no-header", global = true, help_heading = Some("Global FLAGS"))]
    pub no_header: bool,

    /// Set delimiter for input csv file, e.g., in linux -d $'\t' for tab, in powershell -d `t for tab
    #[arg(short = 'd', long = "delimiter", default_value_t = ',', global = true, value_name = "CHAR", help_heading = Some("Global Arguments"))]
    pub delimiter: char,
    /// Set delimiter for output CSV file, e.g., in linux -D $'\t' for tab, in powershell -D `t for tab
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

    /// prints help information
    #[arg(short = 'h', long, action = ArgAction::Help, global= true, help_heading = Some("Global FLAGS"))]
    pub help: Option<String>,

    /// prints version information
    #[arg(short = 'V', long, action = ArgAction::Version, global= true, help_heading = Some("Global FLAGS"))]
    pub version: Option<String>,
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
        #[arg(short = 'c', long = "col-index", value_name = "STR", default_value_t = String::from("1"))]
        col_index: String,
        /// invert the sense of matching, to select non-matching fields
        #[arg(short = 'u', long = "invert-match", help_heading = Some("FLAGS"))]
        invert: bool,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    ///  flattened view of CSV records
    #[command(visible_alias = "flat")]
    flatten {
        /// If enabled, specify characters to write after each record. e.g, "#"
        #[arg(short = 's', long = "separator", value_name = "CHAR")]
        separator: Option<char>,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Build frequency table of selected column in CSV data
    freq {
        /// Select columns index, e.g -c 2,3,5
        #[arg(short = 'c', long = "col-index", value_name = "STR", default_value_t = String::from("1"))]
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
    #[command(visible_alias = "prt")]
    pretty {
        /// Set the whole table width
        #[arg(short = 'w', long = "width-table", value_name = "INT", value_parser = value_parser!(u16).range(0..=65535))]
        width_table: Option<u16>,
        /// If set, truncate content of cells which occupies more than INT lines of space
        #[arg(short = 't', long = "truncate", value_name = "INT")]
        cell_height: Option<usize>,
        /// Set the alignment of content for each cell, possible values: {left, center, right}
        #[arg(short ='a', long = "aln", value_name = "STR", default_value_t = String::from("left"))]
        aln: String,
        /// Show header in different style
        #[arg(long = "header", help_heading = Some("FLAGS"))]
        header: bool,
    },

    /// Replace data of matched fields
    replace {
        /// Select columns index, e.g -c 2,3,5
        #[arg(short = 'c', long = "col-index", value_name = "STR", default_value_t = String::from("1"))]
        col_index: String,
        /// Raw cell content
        #[arg(short = 's', long = "src", value_name = "STR")]
        src: String,
        /// New cell content
        #[arg(short = 'd', long = "dst", value_name = "STR")]
        dst: String,
        /// If set, replace data in whole CSV file, overwrite option -c
        #[arg(short = 'a', long = "all", help_heading = Some("FLAGS"))]
        all: bool,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Reverses rows of CSV data
    #[command(visible_alias = "rev")]
    reverse {
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Randomly select rows from CSV file using reservoir sampling
    sample {
        /// Set subset number
        #[arg(short = 'n', long = "num", value_name = "INT", default_value_t = 10)]
        num: usize,
        /// Set rand seed
        #[arg(short = 's', long = "seed", value_name = "INT", default_value_t = 11)]
        seed: u64,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Applies the regex to each field individually and shows only matching rows.
    search {
        /// Set regex pattern. e.g, "-r \d+"
        #[arg(short = 'r', long = "re", value_name = "STR")]
        pat: String,
        /// If specified, enable case insensitive matching for the entire pattern
        #[arg(short = 'i', long = "ignore-case", help_heading = Some("FLAGS"))]
        case: bool,
        /// invert the sense of matching, to select non-matching rows
        #[arg(short = 'u', long = "invert-match", help_heading = Some("FLAGS"))]
        invert: bool,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Slice rows from a part of a CSV file
    slice {
        /// Skip first int records
        #[arg(short = 's', long = "skip", value_name = "INT", default_value_t = 0)]
        skip: usize,
        /// The length of the slice
        #[arg(short = 'l', long = "len", value_name = "INT", default_value_t = 10)]
        len: usize,
        /// If set, show prefix number in output, start from 1
        #[arg(long = "num", help_heading = Some("FLAGS"))]
        pre_num: bool,
        /// If set, show raw order number in output
        #[arg(long = "raw", help_heading = Some("FLAGS"))]
        raw_order: bool,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
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

    /// Transpose CSV data
    #[command(visible_alias = "trans")]
    transpose {
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Unique data with keys
    uniq {
        /// Select these fields as keys. e.g -k 2,3,5
        #[arg(short = 'k', long = "key", value_name = "STR", default_value_t = String::from("1"))]
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
        #[arg(
            short = 'i',
            long = "sheet-index",
            value_name = "INT",
            default_value_t = 1
        )]
        idx: usize,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out-csv", value_name = "FILE")]
        csv: Option<PathBuf>,
    },

    /// Show CSV file content
    view {
        /// Skip first N records, not include the header row when option "--no-header" enabled. eg "-s 10 --no-header" will skip 11 records
        #[arg(short = 's', long = "skip", value_name = "INT", default_value_t = 0)]
        skip: usize,
        /// If enabled, truncate each record to N fields, if N is greater than the number of fields in this record, then this has no effect
        #[arg(short = 't', long = "truncate", value_name = "INT")]
        truncate: Option<usize>,
        /// Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
        #[arg(short = 'o', long = "out", value_name = "FILE")]
        output: Option<PathBuf>,
    },
}
