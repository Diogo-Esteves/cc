// Word Counter written in Rust, for learning purposes.
// Use this code with at your own risk.

use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::io::BufRead;

fn main() {
   let args: Vec<String> = env::args().collect();

    let (operation, file_path) = parse_config(&args);
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);

    match operation {
        "-c" => println!("{} {}", file_size(path), file_path),
        "-l" => println!("{} {}", lines(&mut reader), file_path),
        "-w" => println!("{} {}", words(&mut reader), file_path),
        "-m" => println!("{} {}", chars(&mut reader), file_path),
        "-" => println!("{} {} {} {}", lines(&mut reader) , words(&mut reader) , chars(&mut reader), file_path),
        _ => panic!("Operation not recognized"),
    };
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let operation = &args[1];
    let filename = &args[2];
    (operation, filename)
}

fn file_size(path: &Path) -> u64 {
    fs::metadata(path).unwrap().len()
}

fn lines(reader: &mut std::io::BufReader<File>) -> u64 {
    let mut total_lines = 0;
    total_lines += reader.lines().count() as u64;
    total_lines
}

fn words(reader: &mut std::io::BufReader<File>) -> u64 {
    let mut total_words = 0;
    for line in reader.lines() {
        total_words += line.unwrap().split_whitespace().count() as u64;
    }
    total_words
}

fn chars(reader: &mut std::io::BufReader<File>) -> usize {
    let mut total_chars = 0;
    for line in reader.lines() {
        total_chars += line.unwrap().chars().count();
    }
    total_chars
}

