use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::fs::File;

pub fn exhaustive_iterator(path_to_file: String) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

// pub fn waiting_update_iterator<F>(path_to_file: String,
//                                wait_update_ms: u64,
//                                stop_when: F) -> io::Result<Lines<BufReader<File>>>
//     where F: Fn() -> bool {
//     panic!("Not implemented!")
// }