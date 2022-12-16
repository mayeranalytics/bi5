![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/mayeranalytics/bi5?include_prereleases)
![GitHub](https://img.shields.io/github/license/mayeranalytics/bi5)
[![Latest Version](https://img.shields.io/crates/v/bi5.svg)](https://crates.io/crates/bi5)  
[![Lib.rs](https://img.shields.io/badge/lib.rs-v0.1.0-blue)](https://lib.rs/crates/bi5)
[![Docs.rs](https://docs.rs/bi5/badge.svg)](https://docs.rs/bi5)
![CI](https://github.com/mayeranalytics/bi5/actions/workflows/ci.yml/badge.svg)
[![Changelog](https://img.shields.io/badge/changelog-0.1.0-blue)](https://github.com/mayeranalytics/bi5/blob/main/Changelog.md)
[![Star](https://img.shields.io/github/stars/mayeranalytics/bi5.svg?style=social&amp;label=Star&amp;maxAge=2592000)](https://github.com/mayeranalytics/bi5)

# bi5

Library and CLI utility for parsing `bi5` tick files.

Bi5 is a simple file format for storing tick data (see [below](bi5-format)). The format is used by the swiss broker [dukascopy](https://www.dukascopy.com/trading-tools/widgets/quotes/historical_data_feed), for example.

## Usage

`read_bi5_file` reads a single file and returns `Vec<Tick>` or `Error`.

```Rust
use bi5::*;
let ticks = read_bi5_file("test/test.bi5", None).expect("Read failed");
assert_eq!(
    ticks.first(), 
    Some(&Tick { millisecs: 1860002, ask: 133153, bid: 133117, askvol: 0.015, bidvol: 0.02 })
);
```

Bi5 files and directories can also be read using an iterator:

```Rust
use bi5::*;
let bi5 = Bi5::new("test/test.bi5", None);
for (date_time, tick) in bi5.iter().expect("File error") {
     println!("{},{}", date_time, tick);
}
```

Bi5 files only contain a time offset. If the base date/time is known it can be
passed to the constructor

```Rust
let bi5 = Bi5::new("test/test.bi5", Some(date_time));
```



## catbi5 utility

The `catbi5` utility dumps a `bi5` tick file to stdout.

```markdown
Usage: catbi5 [OPTIONS] <FILE>

Arguments:
  <FILE>  Filename

Options:
  -d, --date <DATE_TIME>  Date in yyyy-mm-ddTHH:MM:SS format
  -s, --sep <SEP>         Separator [default: "\t"]
  -c, --count             Count ticks
  -h, --help              Print help information
  -V, --version           Print version information
```

When no date is provided the output is based of `0000-01-01T00:00:00`. Otherwise the proper datetime is calculated from from the date input.

When output of `catbi5 test/test.bi5 -d2022-12-16T14:00:00 -s, | head -4`, for example, looks like this

```markdown
t,bid,ask,bidsize,asksize
2022-12-16 14:31:00.002,133117,133153,0.02,0.015
2022-12-16 14:31:00.124,133128,133133,0.000043,0.0075
2022-12-16 14:31:00.174,133067,133103,0.02,0.015
```

## bi5 Format

A bi5 file is a lzma encoded sequence of ticks, where each tick is encoded as follows:

| Field     | Format | Description                    |
| --------- | ------ | ------------------------------ |
| millisecs | u32    | Milliseconds since epoch start |
| ask       | u32    | Ask price                      |
| bid       | u32    | Bid price                      |
| askvol    | f32    | Ask size                       |
| bidvol    | f32    | Bid size                       |

All fields are big-endian encoded.
