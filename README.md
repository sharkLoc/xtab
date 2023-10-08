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
CSV command line utilities

Usage: xtab [OPTIONS] <COMMAND>

Commands:
  view       Show csv content
  addheader  Set new header for CSV file
  head       Get first N records from CSV file
  help       Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet    be quiet and do not show extra information
  -h, --help     Print help
  -V, --version  Print version
```
