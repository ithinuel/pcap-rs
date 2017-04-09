#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate nom;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Result};

use nom::{IResult, le_i32, le_u32, le_u16, be_i32, be_u32, be_u16, Endianness};

#[derive(PartialEq, Debug)]
pub struct Header {
    major: u16,
    minor: u16,
    this_zone: i32,
    sigfigs: u32,
    snaplen: u32,
    network: u32,
    nano_sec: bool
}

#[derive(PartialEq, Debug)]
pub struct Record {
    ts_sec: u32,
    ts_usec: u32,
    orig_len: u32,
    data: Vec<u8>
}

pub enum Block {
    Header(Header),
    Record(Record)
}

#[derive(PartialEq)]
pub enum NextBlock {
    Header,
    Record,
    None
}

pub struct PcapIterator {
    state: NextBlock,
    reader: BufReader<File>,
    endianness: Endianness
}

named_args!(parse_header_e(e: Endianness, nsec: bool)<(Endianness, Header)>, tuple!(
    value!(e),
    do_parse!(
        major: u16!(e) >>
        minor: u16!(e) >>
        this_zone: i32!(e) >>
        sigfigs: u32!(e) >>
        snaplen: u32!(e) >>
        network: u32!(e) >>
        (Header {
            major: major,
            minor: minor,
            this_zone: this_zone,
            sigfigs: sigfigs,
            snaplen: snaplen,
            network: network,
            nano_sec: nsec
        })
    )
));

named!(parse_header<(Endianness, Header)>, switch!(be_u32,
    0xa1b2c3d4 => call!(parse_header_e, Endianness::Big, false)    | // straight sec
    0xd4c3b2a1 => call!(parse_header_e, Endianness::Little, false) | // reverse  sec
    0xa1b23c4d => call!(parse_header_e, Endianness::Big, true)     | // straight usec
    0x4d3cb2a1 => call!(parse_header_e, Endianness::Little, true)    // reverse  usec
));

named_args!(parse_record(e: Endianness)<Record>, do_parse!(
    ts_sec: u32!(e) >>
    ts_usec: u32!(e) >>
    incl_len: u32!(e) >>
    orig_len: u32!(e) >>
    data: take!(incl_len) >>

    (Record {
        ts_sec: ts_sec,
        ts_usec: ts_usec,
        orig_len: orig_len,
        data: Vec::from(data)
    })
));

impl PcapIterator {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<PcapIterator> {
        let f = File::open(path);
        return match f {
            Ok(f) => {
                Ok(
                    PcapIterator {
                        state: NextBlock::Header,
                        reader: BufReader::new(f),
                        endianness: Endianness::Big
                    }
                )
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}

impl Iterator for PcapIterator {
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        if self.state == NextBlock::None {
            return None
        }

        let res = {
            if let Ok(b) = self.reader.fill_buf() {
                match self.state {
                    NextBlock::Header => {
                        match parse_header(b) {
                            IResult::Done(unparsed, (endianness, header)) => {
                                self.endianness = endianness;
                                Some((b.len() - unparsed.len(), Block::Header(header)))
                            }, _ => None
                        }
                    },
                    NextBlock::Record => {
                        None
                    }, NextBlock::None => None
                }
            } else {
                None
            }
        };

        if let Some((len, block)) = res  {
            self.state = NextBlock::Record;
            self.reader.consume(len);
            Some(block)
        } else {
            self.state = NextBlock::None;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use nom;
    use super::*;

    #[test]
    fn parse_header_be_usec() {
        let i = b"\xa1\xb2\xc3\xd4\x00\x02\x00\x04\xFF\xFF\xFF\xFF\
                  \x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: 1,
            nano_sec: false
        };
        assert_eq!(parse_header(&i[..]), nom::IResult::Done(&[10][..], (Endianness::Big, h)));
    }
    #[test]
    fn parse_header_le_usec() {
        let i = b"\xd4\xc3\xb2\xa1\x02\x00\x04\x00\xFF\xFF\xFF\xFF\
                  \x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: 1,
            nano_sec: false
        };
        assert_eq!(parse_header(&i[..]), nom::IResult::Done(&[10][..], (Endianness::Little, h)));
    }

    #[test]
    fn parse_header_be_nsec() {
        let i = b"\xa1\xb2\x3c\x4d\x00\x02\x00\x04\xFF\xFF\xFF\xFF\
                  \x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: 1,
            nano_sec: true
        };
        assert_eq!(parse_header(&i[..]), nom::IResult::Done(&[10][..], (Endianness::Big, h)));
    }
    #[test]
    fn parse_header_le_nsec() {
        let i = b"\x4d\x3c\xb2\xa1\x02\x00\x04\x00\xFF\xFF\xFF\xFF\
                  \x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a";
        let h = Header {
            major: 2,
            minor: 4,
            this_zone: -1,
            sigfigs: 1,
            snaplen: 1,
            network: 1,
            nano_sec: true
        };
        assert_eq!(parse_header(&i[..]), nom::IResult::Done(&[10][..], (Endianness::Little, h)));
    }

    #[test]
    fn parse_record_be_empty() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\
                  \x00\x00\x00\x00\x0a";
        let r = Record {
            ts_sec: 1,
            ts_usec: 0,
            orig_len: 0,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Big), nom::IResult::Done(&[10][..], r));
    }
    #[test]
    fn parse_record_be_some_orig_data_zero_incl() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x00\
                  \x00\x00\x00\x03\x0a";
        let r = Record {
            ts_sec: 1,
            ts_usec: 2,
            orig_len: 3,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Big), nom::IResult::Done(&[10][..], r));
    }
    #[test]
    fn parse_record_be_some_orig_data_parially_incl() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x02\
                  \x00\x00\x00\x03\x0a\x0b\x80";
        let r = Record {
            ts_sec: 1,
            ts_usec: 2,
            orig_len: 3,
            data: vec![10, 11]
        };
        assert_eq!(parse_record(&i[..], Endianness::Big), nom::IResult::Done(&[128][..], r));
    }
    #[test]
    fn parse_record_be_all_data_incl() {
        let i = b"\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x03\
                  \x00\x00\x00\x03\x0a\x0b\x0c\x80";
        let r = Record {
            ts_sec: 1,
            ts_usec: 2,
            orig_len: 3,
            data: vec![10, 11, 12]
        };
        assert_eq!(parse_record(&i[..], Endianness::Big), nom::IResult::Done(&[128][..], r));
    }

    #[test]
    fn parse_record_le_empty() {
        let i = b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
                  \x00\x00\x00\x00\x0a";
        let r = Record {
            ts_sec: 1,
            ts_usec: 0,
            orig_len: 0,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Little), nom::IResult::Done(&[10][..], r));
    }
    #[test]
    fn parse_record_le_some_orig_data_zero_incl() {
        let i = b"\x01\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\
                  \x03\x00\x00\x00\x0a";
        let r = Record {
            ts_sec: 1,
            ts_usec: 2,
            orig_len: 3,
            data: Vec::new()
        };
        assert_eq!(parse_record(&i[..], Endianness::Little), nom::IResult::Done(&[10][..], r));
    }
    #[test]
    fn parse_record_le_some_orig_data_parially_incl() {
        let i = b"\x01\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\
                  \x03\x00\x00\x00\x0a\x0b\x80";
        let r = Record {
            ts_sec: 1,
            ts_usec: 2,
            orig_len: 3,
            data: vec![10, 11]
        };
        assert_eq!(parse_record(&i[..], Endianness::Little), nom::IResult::Done(&[128][..], r));
    }
    #[test]
    fn parse_record_le_all_data_incl() {
        let i = b"\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\
                  \x03\x00\x00\x00\x0a\x0b\x0c\x80";
        let r = Record {
            ts_sec: 1,
            ts_usec: 2,
            orig_len: 3,
            data: vec![10, 11, 12]
        };
        assert_eq!(parse_record(&i[..], Endianness::Little), nom::IResult::Done(&[128][..], r));
    }
}
