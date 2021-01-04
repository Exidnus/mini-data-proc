use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::fs::File;
use std::fs;
use notify::{DebouncedEvent, RecommendedWatcher, Watcher, watcher, RecursiveMode};
use std::time::Duration;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn exhaustive_iterator(path_to_file: String) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

pub fn test() {
    let executor = ThreadPool::new(4);

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch("/home/dvarygin/test2/test_watcher", RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) =>
                if let DebouncedEvent::Create(path_to_file) = event {
                    match path_to_file.into_os_string().into_string() {
                        Ok(str_path_to_file) =>
                            executor.execute(move || { read_to_stdout_with_logging(str_path_to_file) }),
                        Err(os_string) =>
                            println!("Can't convert {:?} to string.", os_string)
                    }
                } else {
                    println!("{:?}", event)
                }
            Err(e) => println!("error: {:?}", e)
        }
    }
}

fn read_to_stdout_with_logging(path_to_file: String) {
    let result = read_to_stdout(path_to_file);
    if let Err(e) = result {
        println!("Error occurred: {:?}", e)
    }
}

fn read_to_stdout(path_to_file: String) -> io::Result<()> {
    let result = fs::read_to_string(path_to_file)?;
    println!("Content:\n {}", result);

    Ok(())
}

// pub fn waiting_update_iterator<F>(path_to_file: String,
//                                wait_update_ms: u64,
//                                stop_when: F) -> io::Result<Lines<BufReader<File>>>
//     where F: Fn() -> bool {
//     panic!("Not implemented!")
// }
