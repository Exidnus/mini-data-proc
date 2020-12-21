use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::fs::File;
use notify::{RecommendedWatcher, Watcher, watcher, RecursiveMode};
use std::time::Duration;
use std::sync::mpsc::channel;

pub fn exhaustive_iterator(path_to_file: String) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

pub fn test() {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch("/home/dmitriy_varygin/test/test_watcher", RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("error: {:?}", e)
        }
    }
}

// pub fn waiting_update_iterator<F>(path_to_file: String,
//                                wait_update_ms: u64,
//                                stop_when: F) -> io::Result<Lines<BufReader<File>>>
//     where F: Fn() -> bool {
//     panic!("Not implemented!")
// }
