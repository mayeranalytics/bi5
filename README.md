![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/mayeranalytics/bi5?include_prereleases)
![GitHub](https://img.shields.io/github/license/mayeranalytics/bi5)
[![Latest Version](https://img.shields.io/crates/v/bi5.svg)](https://crates.io/crates/bi5)  [![Docs.rs](https://docs.rs/bi5/badge.svg)](https://docs.rs/bi5)
[![Lib.rs](https://img.shields.io/badge/lib.rs-v0.1.0-blue)](https://lib.rs/crates/bi5)
![CI](https://github.com/mayeranalytics/bi5/actions/workflows/ci.yml/badge.svg)
[![Changelog](https://img.shields.io/badge/changelog-0.1.0-blue)](https://github.com/mayeranalytics/bi5/blob/main/Changelog.md)
[![Star](https://img.shields.io/github/stars/mayeranalytics/bi5.svg?style=social&amp;label=Star&amp;maxAge=2592000)](https://github.com/mayeranalytics/bi5)

# bi5

Parse bi5 tick files.

Bi5 is a simple file format for storing tick data.

## Usage

```Rust
use bi5::*;
let ticks = read_bi5("test/test.bi5").expect("Read failed");
assert_eq!(
    ticks.first(), 
    Some(&Tick { millisecs: 1860002, ask: 133153, bid: 133117, askvol: 0.015, bidvol: 0.02 })
);
```

## catbi5 utility

The `catbi5` utility dumps a `bi5` file to stdout.

```markdown
Usage: catbi5 [OPTIONS] <FILE>

Arguments:
  <FILE>  Filename

Options:
  -d, --date <DATE_TIME>  Date in yyyy-mm-ddTHH:MM:SS format
  -s, --sep <SEP>         Separator [default: "\t"]
  -h, --help              Print help information
  -V, --version           Print version information
```

When no date is provided the output shows the milliseconds. Otherwise the proper datetime is calculated and shown.

When output of `catbi5 test/test.bi5 -d2022-12-15T14:00:00 -s, | head -4`, for example, looks like this

```markdown
t,bid,ask,bidvol,askvol
2022-12-15 14:31:00.002,133117,133153,0.02,0.015
2022-12-15 14:31:00.124,133128,133133,0.000043,0.0075
2022-12-15 14:31:00.174,133067,133103,0.02,0.015
2022-12-15 14:31:00.265,133078,133102,0.00036,0.015
```

## bi5 Format

A bi5 file is a lzma encoded sequence of ticks, where each tick is encoded as follows:

| Field     | Format | Description                    |
| --------- | ------ | ------------------------------ |
| millisecs | u32    | Milliseconds since epoch start |
| ask       | u32    | Ask price                      |
| bid       | u32    | Bid price                      |
| askvol    | f32    | Ask volume                     |
| bidvol    | f32    | Bid volume                     |

All fields are big-endian encoded.
