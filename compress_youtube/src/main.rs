extern crate flate2;

// Importing encoder module
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main(){
    // Creating a GzEncoder
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
}