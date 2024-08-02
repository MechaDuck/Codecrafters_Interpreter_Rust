mod tokenizer;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use tokenizer::tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        process::exit(64); // Exit code for incorrect usage
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        process::exit(66); // Exit code for file read error
    });


    if command == "tokenize" {
        tokenize(file_contents);
    } else if command == "parse" {

    }
    writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
    process::exit(64); // Exit code for unknown command
}


fn tokenize(file_contents: String){
    let mut tokenizer = Tokenizer::new();
    
    for line in file_contents.lines() {
        tokenizer.tokenize(line);
    }

    for token in tokenizer.tokens {
        token.print();
    }

    if tokenizer.found_error {
        std::process::exit(65);
    }

    std::process::exit(0);
}

fn parse(file_contents: String) {
    let mut tokenizer = Tokenizer::new();
    
    for line in file_contents.lines() {
        tokenizer.tokenize(line);
    }
}