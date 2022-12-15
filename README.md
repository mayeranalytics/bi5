![GitHub](https://img.shields.io/github/license/mayeranalytics/bi5)
![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/mayeranalytics/bi5?include_prereleases)
![CI](https://github.com/mayeranalytics/bi5/actions/workflows/ci.yml/badge.svg)

# bi5

Parse bi5 files.

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
