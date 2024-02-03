// Word Counter written in Rust, for learning purposes.
// Use this code with at your own risk.

use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::io::{BufRead, BufReader};

fn main() {
   let args: Vec<String> = env::args().collect();

    let (operation, file_path) = parse_config(&args);
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

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

fn lines(reader: &mut BufReader<File>) -> u64 {
    let mut total_lines = 0;
    total_lines += reader.lines().count() as u64;
    total_lines
}

fn words(reader: &mut BufReader<File>) -> u64 {
    let mut total_words = 0;
    for line in reader.lines() {
        total_words += line.unwrap().split_whitespace().count() as u64;
    }
    total_words
}

fn chars(reader: &mut BufReader<File>) -> u64 {
    let mut total_chars = 0;
    let mut buffer = String::new();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        total_chars += buffer.chars().count() as u64;
        buffer.clear();
    }
    total_chars
}

// write unit tests for the functions above
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let args = vec![
            String::from("word_counter"),
            String::from("-c"),
            String::from("test.txt"),
        ];
        assert_eq!(parse_config(&args), ("-c", "test.txt"));
    }

    #[test]
    fn test_file_size() {
        let path = Path::new("test.txt");
        assert_eq!(file_size(path), 342190);
    }

    #[test]
    fn test_lines() {
        let file = File::open("test.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert_eq!(lines(&mut reader), 7145);
    }

    #[test]
    fn test_words() {
        let file = File::open("test.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert_eq!(words(&mut reader), 58164);
    }

    #[test]
    fn test_chars() {
        let file = File::open("test.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert_eq!(chars(&mut reader), 339292);
    }
}

