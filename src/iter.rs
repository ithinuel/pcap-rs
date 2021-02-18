//! A nice and simple iterator over the file.
use crate::{parse_header, parse_record, Header, Record};
use nom::{error::ErrorKind, number::Endianness, Err};
use std::io::BufRead;

/// Represents an entry in the file.
#[derive(PartialEq, Debug)]
pub enum Block {
    /// Header of the file.
    ///
    /// This type of entry occurs only once at the beginning of each file.
    Header(Header),
    /// Packet record.
    ///
    /// This type of entry occurs until the end of the file is reached.
    Record(Record),

    /// Parsing error.
    Error(ErrorKind),
}

/// Iterates over a BufRead.
pub struct PcapIterator<T: BufRead> {
    first_block: bool,
    reader: T,
    endianness: Endianness,
    nano_sec: bool,
}
impl<T: BufRead> PcapIterator<T> {
    pub fn new(stream: T) -> PcapIterator<T> {
        PcapIterator {
            first_block: true,
            reader: stream,
            endianness: Endianness::Big,
            nano_sec: false,
        }
    }
}

impl<T: BufRead> Iterator for PcapIterator<T> {
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        // these are required to avoid borrowing self inside the first closure while it is still
        // mutable borrowed by `self.read.fill_buf()`.
        let first_block = self.first_block;
        let endianness = self.endianness;
        let nano_sec = self.nano_sec;

        self.reader
            .fill_buf()
            .and_then(|b| {
                (if first_block {
                    parse_header(b).map(|(u, h)| Some((b.len() - u.len(), Block::Header(h))))
                } else {
                    parse_record(b, endianness, nano_sec)
                        .map(|(u, r)| Some((b.len() - u.len(), Block::Record(r))))
                })
                .or_else(|e| match e {
                    Err::Incomplete(_)
                    | Err::Error((_, ErrorKind::Complete))
                    | Err::Failure((_, ErrorKind::Complete)) => Ok(None),
                    Err::Error((_, e)) | Err::Failure((_, e)) => Ok(Some((0, Block::Error(e)))),
                })
            })
            .unwrap_or(None)
            .map(|(c, block)| {
                self.first_block = false;
                if c != 0 {
                    self.reader.consume(c);
                }
                if let Block::Header(ref h) = block {
                    self.endianness = h.endianness;
                    self.nano_sec = h.nano_sec;
                }
                block
            })
    }
}
