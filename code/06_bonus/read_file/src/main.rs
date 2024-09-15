// use std::fs::read_to_string;

// fn main() {
//     // Herbert's benchmarking shows that: 
//     // 1GB file read in 2,311ms
//     let now = std::time::Instant::now();
//     let war_and_peace = read_to_string("../../03_async/buffered_reader/warandpeace.txt").unwrap();
//     println!("Line count: {}", war_and_peace.lines().count());
//     println!("Complete in {} ms", now.elapsed().as_millis());
// }

// use std::{
//     fs::File,
//     io::{BufRead, BufReader},
// };

// fn main() {
//     // Herbert's benchmarking shows that:
//     // 1GB file read in 853ms
//     let now = std::time::Instant::now();
//     let file = File::open("../../03_async/buffered_reader/warandpeace.txt").unwrap();
//     let buffered_reader = BufReader::new(file);
//     println!("Line count: {}", buffered_reader.lines().count());
//     println!("Complete in {} ms", now.elapsed().as_millis());
// }

use memmap2::MmapOptions;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Herbert's benchmarking shows that:
    // 1GB file read in 701ms this can be further reduced by using threads
    let now = std::time::Instant::now();
    let file = File::open("../../03_async/buffered_reader/warandpeace.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let buffered_reader = BufReader::new(mmap.as_ref());
    println!("Line count: {}", buffered_reader.lines().count());
    println!("Complete in {} ms", now.elapsed().as_millis());
}
