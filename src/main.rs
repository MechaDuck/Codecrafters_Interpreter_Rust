use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut found_error = false;
            let mut line_number = 1;
            for line in file_contents.lines() {
                let chars: Vec<char> = line.chars().collect();
                let mut i = 0;
        
                while i < chars.len() {
                    match chars[i] {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        '{' => println!("LEFT_BRACE {{ null"),
                        '}' => println!("RIGHT_BRACE }} null"),
                        '*' => println!("STAR * null"),
                        ',' => println!("COMMA , null"),
                        '.' => println!("DOT . null"),
                        '+' => println!("PLUS + null"),
                        '-' => println!("MINUS - null"),
                        ';' => println!("SEMICOLON ; null"),
                        '=' => {
                            if i + 1 < chars.len() && chars[i + 1] == '=' {
                                println!("EQUAL_EQUAL == null");
                                i += 1; // Skip the next character as it's part of the double equal sign
                            } else {
                                println!("EQUAL = null");
                            }
                        },
                        '!' => {
                            if i + 1 < chars.len() && chars[i + 1] == '=' {
                                println!("BANG_EQUAL != null");
                                i += 1; // Skip the next character as it's part of the double equal sign
                            } else {
                                println!("BANG ! null");
                            }
                        },
                        '<' => {
                            if i + 1 < chars.len() && chars[i + 1] == '=' {
                                println!("LESS_EQUAL <= null");
                                i += 1; // Skip the next character as it's part of the double equal sign
                            } else {
                                println!("LESS < null");
                            }
                        },
                        '>' => {
                            if i + 1 < chars.len() && chars[i + 1] == '=' {
                                println!("GREATER_EQUAL >= null");
                                i += 1; // Skip the next character as it's part of the double equal sign
                            } else {
                                println!("GREATER > null");
                            }
                        },
                        '/' => {
                            if i + 1 < chars.len() && chars[i + 1] == '/' {
                                break;
                            } else {
                                println!("SLASH / null");
                            }
                        },
                        '\t' => {/* Ignore */},
                        ' ' => {/* Ignore */},
                        _ => { 
                            eprintln!("[line {}] Error: Unexpected character: {}", line_number, chars[i]);
                            found_error = true;
                        },
                    }
                    i += 1;
                }
                line_number += 1;
            }
            println!("EOF  null");
            if found_error {
                process::exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
