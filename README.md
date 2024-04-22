# xtab
🦀 CSV command line utilities


## install
##### setp1：install cargo first 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### step2:
```bash
cargo install xtab
# or

git clone https://github.com/sharkLoc/xtab.git
cd xtab
cargo b --release
# mv target/release/xtab to anywhere you want 
```

## usage

```bash
xtab -- CSV command line utilities
Version: 0.0.2

Authors: sharkLoc <mmtinfo@163.com>
Source code: https://github.com/sharkLoc/xtab.git

xtab supports reading and writing gzip/bzip2/xz format file.
Compression level:
  format   range   default   crate
  gzip     1-9     6         https://crates.io/crates/flate2
  bzip2    1-9     6         https://crates.io/crates/bzip2
  xz       1-9     6         https://crates.io/crates/xz2


Usage: xtab [OPTIONS] [CSV] <COMMAND>

Commands:
  addheader  Set new header for CSV file [aliases: ah]
  dim        Dimensions of CSV file
  head       Get first N records from CSV file
  uniq       Unique data without sorting
  view       Show CSV file content
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

Global Arguments:
  -o, --out <FILE>            Output file name, file ending in .gz/.bz2/.xz will be compressed automatically, if file not specified write data to stdout
  -d, --delimiter <CHAR>      Set delimiter for input csv file, e.g., -d $'\t' for tab [default: ,]
  -D, --out-delimite <CHAR>   Set delimiter for output CSV file, e.g., -D $'\t' for tab [default: ,]
      --log <FILE>            If file name specified, write log message to this file, or write to stderr
      --compress-level <INT>  Set compression level 1 (compress faster) - 9 (compress better) for gzip/bzip2/xz output file, just work with option -o/--out [default: 6]
  -v, --verbosity <STR>       Control verbosity of logging, possible values: {error, warn, info, debug, trace} [default: debug]
  [CSV]                   Input csv file name, if file not specified read data from stdin

Global FLAGS:
  -H, --no-header  If set, the first row is treated as a special header row, and the original header row excluded from output
  -q, --quiet      Be quiet and do not show any extra information

Use "xtab help [command]" for more information about a command
```
