extern crate pcap_rs;

use pcap_rs::iter::PcapIterator;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <foo.pcap> ...", args[0]);
        return;
    }

    for path in args.iter().skip(1) {
        let f = File::open(path).unwrap();

        for b in PcapIterator::new(BufReader::new(f)) {
            println!("{:?}", b);
        }
    }
}
