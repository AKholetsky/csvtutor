extern crate csv;

use std::io;

fn main() {
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.records() {
        let record = result.expect("expect valid csv record");
        println!("{:?}", record);
    }
}
