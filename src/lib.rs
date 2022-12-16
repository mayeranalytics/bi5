//! Parse bi5 tick files.
//! 
//! The crate provides:
//! - `Tick` struct
//! - `read_bi5_file` function returning a `Vec<Tick>`
//! - `Bi5` struct that provides an iterator `Bi5Iter`
use std::{
    path::{Path, PathBuf},
    fs::File,
    io::{Cursor, BufReader},
    mem::size_of,
    ffi::OsStr,
    fmt,
};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use walkdir::{WalkDir};
use binread::BinRead;
use lzma_rs::lzma_decompress;
use anyhow::{anyhow, Error};

/// `Tick` is the basic building block of a bi5 file.
#[derive(BinRead, Debug, PartialEq)]
pub struct Tick {
    /// Milliseconds since file start (usually encoded in the file path)
    #[br(big)]
    pub millisecs: u32,
    /// Ask price
    #[br(big)]
    pub ask: u32,
    /// Bid price
    #[br(big)]
    pub bid: u32,
    /// Ask size
    #[br(big)]
    pub asksize: f32,
    /// Bid size
    #[br(big)]
    pub bidsize: f32,
}

impl fmt::Display for Tick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{},{},{}", self.millisecs, self.bid, self.ask, self.bidsize, self.asksize)
    }
}

/// Represents a bi5 file or directory
pub struct Bi5 {
    path: PathBuf,
    date_time: NaiveDateTime
}

/// Iterator over bi5 file or directories
/// 
/// ```
/// use bi5::Bi5;
/// let bi5 = Bi5::new("test/test.bi5", None);
/// for (date_time, tick) in bi5.iter().expect("File error") {
///     println!("{},{}", date_time, tick);
/// }
/// ```
pub enum Bi5Iter {
    File {
        cursor: Cursor<Vec<u8>>,
        date_time: NaiveDateTime,
    },
    Dir {
        walk_dir: walkdir::IntoIter,
        file_iter: Box<Bi5Iter>,
        date_time: NaiveDateTime,
    },
    Empty
}

/// Returns 0000-01-01T00:00:00
fn zero_timestamp() -> NaiveDateTime {
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(0, 1, 1).unwrap(),
        NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap()
    )
}

impl Bi5 {

    /// Create `Bi5` representing a bi5 file or directory
    /// ## Arguments
    /// - `path` - File or directory path
    /// - `date_time` - Optional datetime (only meaningful for file)
    pub fn new<P:AsRef<Path>>(path: P, date_time: Option<NaiveDateTime>) -> Self {
        Bi5 { 
            path: path.as_ref().to_path_buf(),
            date_time: date_time.unwrap_or(zero_timestamp())
        }
    }

    /// Returns true if `Bi5` is a file
    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    fn forward_to_next_good_file(walk_dir: &mut walkdir::IntoIter) 
    -> Result<Option<(walkdir::DirEntry, NaiveDateTime)>, Error> {
        loop {
            if let Some(entry) = walk_dir.next() {
                let entry = entry?;
                if let Some(datetime) = entry.path().to_datetime() {
                    return Ok(Some((entry, datetime)));
                } else {
                    continue
                }
            } else {
                return Ok(None);
            }
        }
    }

    /// Returns an iterator or `Error`
    pub fn iter(&self) -> Result<Bi5Iter, Error> 
    {
        
        if self.path.is_file() {

            let file: File = File::open(&self.path)?;
            let file_len: u64 = file.metadata()?.len();

            let mut buf: Vec<u8> = Vec::new();  // buffer to decode into
            if file_len == 0 {
                return Ok(Bi5Iter::File { 
                    cursor: Cursor::new(buf),
                    date_time: self.date_time
                })
            }
            let mut f_reader: BufReader<File> = BufReader::new(file);
            lzma_decompress(&mut f_reader, &mut buf)?;
        
            if buf.len() % size_of::<Tick>() != 0 {
                return Err(anyhow!(
                    "Decompressed buffer length {} is not a multiple of {}", 
                    buf.len(), 
                    size_of::<Tick>()
                ));
            }

            Ok(Bi5Iter::File { 
                cursor: Cursor::new(buf),
                date_time: self.date_time,
            })

        } else if self.path.is_dir() {

            let mut walk_dir = WalkDir::new(&self.path)
                .sort_by_key(direntry_to_key)
                .into_iter();

            if let Some((entry, date_time)) = 
                Self::forward_to_next_good_file(&mut walk_dir)? 
            {
                let file_iter = Bi5::new(entry.path(), Some(date_time)).iter()?;
                return Ok(Bi5Iter::Dir { walk_dir, file_iter: Box::new(file_iter), date_time })
            } else {
                return Ok(Bi5Iter::Empty);
            }
        } else {
            Err(anyhow!("{} must be file or dir", self.path.to_string_lossy()))
        }
    }
}

impl<'a> Iterator for Bi5Iter {
    type Item = (NaiveDateTime, Tick);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Bi5Iter::Empty => { None }
            Bi5Iter::File { cursor, date_time } => {
                Tick::read(cursor).ok().map(|tick|(*date_time, tick))
            },
            Bi5Iter::Dir { walk_dir, file_iter, date_time } => {
                if let Some(tick) = file_iter.next() {
                    return Some(tick)
                } else { // ticks exhausted, get new
                    if let Some((entry, date_time_)) = Bi5::forward_to_next_good_file(walk_dir).ok()? {
                        *date_time = date_time_;
                        *file_iter = Box::new(Bi5::new(entry.path(), Some(date_time_)).iter().ok()?);
                        self.next()
                    } else {
                        None
                    }
                }
            }
        }
    }
}

/// Decompress and parse a bi5 file
/// ## Arguments
/// - `path` - Path to bi5 file
/// - `date_time` - Optional date_time of file
/// ## Returns
/// `Vec` of `Tick`s or `Error`
/// 
/// ## Usage
/// ```
/// use bi5::*;
/// let ticks = read_bi5_file("test/test.bi5", None).expect("Read failed");
/// assert_eq!(
///     ticks.first(),
///     Some(&Tick { millisecs: 1860002, ask: 133153, bid: 133117, asksize: 0.015, bidsize: 0.02 })
/// );
/// ```
pub fn read_bi5_file<P:AsRef<Path>+Copy>(path: P, date_time: Option<NaiveDateTime>) 
    -> Result<Vec<Tick>, Error>
{
    let bi5 = Bi5::new(path, date_time);
    let ticks = bi5.iter()?.map(|x|x.1).collect();
    Ok(ticks)

}

trait ToDateTime {
    fn to_datetime(&self) -> Option<NaiveDateTime>;
}

impl ToDateTime for Path {
    fn to_datetime(&self) -> Option<NaiveDateTime>
    {
        if!self.is_file() { 
            None 
        } else {
            let mut v: Vec<&OsStr> = self.iter().collect();
            let f: &str = v.pop()?.to_str()?;
            if f.len() < 2 { return None; }
            let h: u32 = (&f[0..2]).parse::<u32>().ok()?;
            let d: u32 = v.pop()?.to_str()?.parse::<u32>().ok()?;
            let m: u32 = v.pop()?.to_str()?.parse::<u32>().ok()?;
            let y: u32 = v.pop()?.to_str()?.parse::<u32>().ok()?;
            Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(y as i32, m+1, d)?, 
                NaiveTime::from_hms_opt(h, 0, 0)?
            ))
        }
    }
}

fn direntry_to_key(entry: &walkdir::DirEntry) -> NaiveDateTime {
    entry.path().to_datetime().unwrap_or(zero_timestamp())
}

#[test]
/// Test correct length, and correctness of first and last tick in test/test.bi5
fn test_read_bi5() {
    match read_bi5_file("test/test.bi5", None) {
        Err(_) => assert!(false),
        Ok(ticks) => {
            assert_eq!(ticks.len(), 10412);
            assert_eq!(
                ticks.first(), 
                Some(&Tick { millisecs: 1860002, bid: 133117, ask: 133153, bidsize: 0.02, asksize: 0.015 })
            );
            assert_eq!(
                ticks.last(), 
                Some(&Tick { millisecs: 3599899,  bid: 131427, ask: 131453,bidsize: 0.02, asksize: 0.015 })
            );
        }
    }
}