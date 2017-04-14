use std::io::BufRead;
use nom::{IResult, Endianness, Err};
use ::{Header, parse_header, Record, parse_record};

#[derive(PartialEq)]
pub enum NextBlock {
    Header,
    Record,
    None
}

#[derive(PartialEq, Debug)]
pub enum Block {
    Header(Header),
    Record(Record),
    Error(Err<u32>)
}

pub struct PcapIterator<T: BufRead> {
    state: NextBlock,
    reader: T,
    endianness: Endianness,
    nano_sec: bool
}
impl<T: BufRead> PcapIterator<T> {
    pub fn new(stream: T) -> PcapIterator<T> {
        PcapIterator {
            state: NextBlock::Header,
            reader: stream,
            endianness: Endianness::Big,
            nano_sec: false
        }
    }
}

impl<T: BufRead> Iterator for PcapIterator<T> {
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        let mut ret = None;
        let mut consume = 0;

        if self.state == NextBlock::None {
            return None
        }

        if let Ok(b) = self.reader.fill_buf() {
            match self.state {
                NextBlock::Header => {
                    match parse_header(b) {
                        IResult::Done(unparsed, header) => {
                            self.endianness = header.endianness;
                            self.nano_sec = header.nano_sec;
                            consume = b.len() - unparsed.len();
                            ret = Some(Block::Header(header));
                        },
                        IResult::Error(e) => {
                            self.state = NextBlock::None;
                            ret = Some(Block::Error(e));
                        }, _ => {}
                    }
                },
                NextBlock::Record => {
                    match parse_record(b, self.endianness, self.nano_sec) {
                        IResult::Done(unparsed, record) => {
                            consume = b.len() - unparsed.len();
                            ret = Some(Block::Record(record));
                        },
                        IResult::Error(e) => {
                            self.state = NextBlock::None;
                            ret = Some(Block::Error(e))
                        }, _ => {}
                    }
                }, NextBlock::None => {}
            }
        }

        if consume != 0 {
            self.state = NextBlock::Record;
            self.reader.consume(consume);
        }
        ret
    }
}
