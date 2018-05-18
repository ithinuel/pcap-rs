use nom::{Endianness, ErrorKind};
use std::io::BufRead;
use {parse_header, parse_record, Header, Record};

#[derive(PartialEq)]
pub enum NextBlock {
    Header,
    Record,
    None,
}

#[derive(PartialEq, Debug)]
pub enum Block {
    Header(Header),
    Record(Record),
    Error(ErrorKind),
}

pub struct PcapIterator<T: BufRead> {
    state: NextBlock,
    reader: T,
    endianness: Endianness,
    nano_sec: bool,
}
impl<T: BufRead> PcapIterator<T> {
    pub fn new(stream: T) -> PcapIterator<T> {
        PcapIterator {
            state: NextBlock::Header,
            reader: stream,
            endianness: Endianness::Big,
            nano_sec: false,
        }
    }
}

impl<T: BufRead> Iterator for PcapIterator<T> {
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        let mut ret = None;
        let mut consume = 0;

        if self.state == NextBlock::None {
            return None;
        }

        if let Ok(b) = self.reader.fill_buf() {
            match self.state {
                NextBlock::Header => match parse_header(b) {
                    Ok((unparsed, header)) => {
                        self.endianness = header.endianness;
                        self.nano_sec = header.nano_sec;
                        consume = b.len() - unparsed.len();
                        ret = Some(Block::Header(header));
                    }
                    Err(e) => {
                        self.state = NextBlock::None;
                        ret = Some(Block::Error(e.into_error_kind()));
                    }
                },
                NextBlock::Record => match parse_record(b, self.endianness, self.nano_sec) {
                    Ok((unparsed, record)) => {
                        consume = b.len() - unparsed.len();
                        ret = Some(Block::Record(record));
                    }
                    Err(e) => {
                        self.state = NextBlock::None;
                        ret = Some(Block::Error(e.into_error_kind()))
                    }
                },
                NextBlock::None => {}
            }
        }

        if consume != 0 {
            self.state = NextBlock::Record;
            self.reader.consume(consume);
        }
        ret
    }
}
