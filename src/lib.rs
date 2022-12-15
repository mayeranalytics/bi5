//! Provides
//! - `Tick` struct
//! - `read_bi5` function
use binread::BinRead;
use std::{
    path::Path,
    fs::File,
    io::{Cursor, BufReader},
    mem::size_of,
};
use lzma_rs::lzma_decompress;
use anyhow::{anyhow, Error};

#[derive(BinRead, Debug, PartialEq)]
pub struct Tick {
    #[br(big)]
    pub millisecs: u32,
    #[br(big)]
    pub ask: u32,
    #[br(big)]
    pub bid: u32,
    #[br(big)]
    pub askvol: f32,
    #[br(big)]
    pub bidvol: f32,
}

/// Decompress and parse the bi5 file
/// ## Arguments
/// - `path` - Path to bi5 file
/// ## Returns
/// `Vec` of `Tick`s 
/// 
/// ## Usage
/// ```
/// use bi5::*;
/// let ticks = read_bi5("test/test.bi5").expect("Read failed");
/// assert_eq!(
///     ticks.first(), 
///     Some(&Tick { millisecs: 1860002, ask: 133153, bid: 133117, askvol: 0.015, bidvol: 0.02 })
/// );
/// ```
pub fn read_bi5<P:AsRef<Path>>(path: P) -> Result<Vec<Tick>, Error> {
    let mut f_in: BufReader<File> = BufReader::new(File::open(path)?);
    let mut buf: Vec<u8> = Vec::new();
    lzma_decompress(&mut f_in, &mut buf)?;
    if buf.len() % size_of::<Tick>() != 0 {
        return Err(anyhow!(
            "Decompressed buffer length {} is not a multiple of {}", 
            buf.len(), 
            size_of::<Tick>()
        ));
    }
    let n_ticks: usize = buf.len() / size_of::<Tick>();
    let mut reader: Cursor<&Vec<u8>> = Cursor::new(&buf);
    let mut ticks: Vec<Tick> = Vec::with_capacity(n_ticks);
    for _ in 0..n_ticks {
        ticks.push(Tick::read(&mut reader)?);
    }
    Ok(ticks)
}

#[test]
/// Test correct length, and correctness of first and last tick in test/test.bi5
fn test_read() {
    match read_bi5("test/test.bi5") {
        Err(_) => assert!(false),
        Ok(ticks) => {
            assert_eq!(ticks.len(), 10412);
            assert_eq!(
                ticks.first(), 
                Some(&Tick { millisecs: 1860002, bid: 133117, ask: 133153, bidvol: 0.02, askvol: 0.015 })
            );
            assert_eq!(
                ticks.last(), 
                Some(&Tick { millisecs: 3599899,  bid: 131427, ask: 131453,bidvol: 0.02, askvol: 0.015 })
            );
        }
    }
}