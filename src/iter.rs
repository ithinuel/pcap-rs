//! A nice and simple iterator over the file.
use nom::{Endianness, ErrorKind};
use std::io::BufRead;
use {parse_header, parse_record, Header, Record};

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

        self.reader.fill_buf().and_then(|b| {
            (if first_block {
                parse_header(b).and_then(|(u, h)| {
                    Ok(Some((b.len()-u.len(), Block::Header(h))))
                })
            } else {
                parse_record(b, endianness, nano_sec).and_then(|(u, r)| {
                    Ok(Some((b.len()-u.len(), Block::Record(r))))
                })
            }).or_else(|e| {
                let e = e.into_error_kind();
                if let ErrorKind::Complete = e {
                    Ok(None)
                } else {
                    Ok(Some((0, Block::Error(e))))
                }
            })
        }).unwrap_or(None)
        .and_then(|(c, block)| {
            self.first_block = false;
            if c != 0 {
                self.reader.consume(c);
            }
            if let Block::Header(ref h) = block {
                self.endianness = h.endianness;
                self.nano_sec = h.nano_sec;
            }
            Some(block)
        })
    }
}
