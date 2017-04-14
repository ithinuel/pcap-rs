extern crate pcap;

use std::env;
use std::fs::File;
use std::io::BufReader;
use pcap::iter::PcapIterator;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <foo.pcap> ...", args[0]);
        return;
    }

    for path in args.iter() {
        let f = File::open(path).unwrap();

        for b in PcapIterator::new(BufReader::new(f)) {
            println!("{:?}", b);
        }
    }
}
