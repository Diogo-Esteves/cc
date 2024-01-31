# WC - Word Counter in Rust

This is a [Rust](https://www.rust-lang.org/) program that counts the number of words in a given a path of a file.

## How to run

To run the program, use the following command:

```bash
cargo run -- -<option> <path_to_file>
```

## Accepted parameters for options

- `-c <path_to_file>`: To count the byte size 
- `-l <path_to_file>`: To count the number of lines
- `-w <path_to_file>`: To count the number of words
- `-m <path_to_file>`: To count the number of characters
- `--help`: To show the help menu


## For testing purpose

The file of test should return:
`-c test.txt` -> 342190
`-l test.txt` -> 7145
`-w test.txt` -> 58164
`-mtest.txt` -> 339292
