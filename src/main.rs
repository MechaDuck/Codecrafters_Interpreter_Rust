use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        process::exit(64); // Exit code for incorrect usage
    }

    let command = &args[1];
    let filename = &args[2];

    if command != "tokenize" {
        writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        process::exit(64); // Exit code for unknown command
    }

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        process::exit(66); // Exit code for file read error
    });

    let mut tokenizer = Tokenizer::new();

    for line in file_contents.lines() {
        tokenizer.tokenize(line);
        tokenizer.line_number += 1;
    }

    println!("EOF  null");
    if tokenizer.found_error {
        process::exit(65); // Exit code for parsing errors
    }
}



struct Tokenizer {
    found_error: bool,
    line_number: usize,
}

impl Tokenizer {
    fn new() -> Self {
        Self {
            found_error: false,
            line_number: 1,
        }
    }

    fn tokenize(&mut self, line: &str) {
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '(' => self.print_token("LEFT_PAREN", "("),
                ')' => self.print_token("RIGHT_PAREN", ")"),
                '{' => self.print_token("LEFT_BRACE", "{"),
                '}' => self.print_token("RIGHT_BRACE", "}"),
                '*' => self.print_token("STAR", "*"),
                ',' => self.print_token("COMMA", ","),
                '.' => self.print_token("DOT", "."),
                '+' => self.print_token("PLUS", "+"),
                '-' => self.print_token("MINUS", "-"),
                ';' => self.print_token("SEMICOLON", ";"),
                '=' => self.handle_equal(&chars, &mut i),
                '!' => self.handle_bang(&chars, &mut i),
                '<' => self.handle_less(&chars, &mut i),
                '>' => self.handle_greater(&chars, &mut i),
                '/' => {
                    if self.handle_slash(&chars, &mut i){
                        break;
                    }
                },
                '\t' | ' ' => {}, // Ignore tabs and spaces
                '"' => self.handle_string(&chars, &mut i),
                '0'..='9' => self.handle_number(&chars, &mut i),
                _ => self.handle_unexpected(chars[i]),
            }
            i += 1;
        }
    }

    fn print_token(&self, token_type: &str, value: &str) {
        println!("{} {} null", token_type, value);
    }

    fn handle_equal(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("EQUAL_EQUAL == null");
            *i += 1; // Skip the next character
        } else {
            println!("EQUAL = null");
        }
    }

    fn handle_bang(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("BANG_EQUAL != null");
            *i += 1; // Skip the next character
        } else {
            println!("BANG ! null");
        }
    }

    fn handle_less(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("LESS_EQUAL <= null");
            *i += 1; // Skip the next character
        } else {
            println!("LESS < null");
        }
    }

    fn handle_greater(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("GREATER_EQUAL >= null");
            *i += 1; // Skip the next character
        } else {
            println!("GREATER > null");
        }
    }

    fn handle_slash(&mut self, chars: &[char], i: &mut usize) -> bool {
        if *i + 1 < chars.len() && chars[*i + 1] == '/' {
            // Return true for "Comment detected", so the line can be skipped
            return true;
        } 
        println!("SLASH / null");
        return false;
        

    }

    fn handle_string(&mut self, chars: &[char], i: &mut usize) {
        let mut tmp_string = String::new();
        let mut found_string_end = false;
        *i += 1; // Skip the starting quote

        while *i < chars.len() {
            if chars[*i] == '"' {
                found_string_end = true;
                break;
            }
            tmp_string.push(chars[*i]);
            *i += 1;
        }

        if !found_string_end {
            eprintln!("[line {}] Error: Unterminated string.", self.line_number);
            self.found_error = true;
        } else {
            // Print the string with literal escape sequences
            println!(
                "STRING \"{}\" {}",
                tmp_string,
                tmp_string.trim_matches('"')
            );
        }
    }

    fn handle_unexpected(&mut self, c: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", self.line_number, c);
        self.found_error = true;
    }

    fn handle_number(&mut self, chars: &[char], i: &mut usize) {
        let mut number_str = String::new();
        let mut decimal_found = false;
        // Collect digits to form the number
        while *i < chars.len() && (chars[*i].is_digit(10) || chars[*i] == '.') {
            if chars[*i] == '.'{
                if decimal_found{
                    // TODO: ERROR HANDLING
                    break;
                }
                decimal_found = true;
            }
            number_str.push(chars[*i]);
            *i += 1;
        }

        // Decrease `i` by 1 to counter the extra increment in the loop
        *i -= 1;

        if decimal_found && number_str.ends_with('.') {
            let number_without_dot = number_str.trim_end_matches('.');
            let float_number_str = format!("{}0", number_without_dot);
            println!("NUMBER {} (without trailing dot) {}", number_without_dot, float_number_str);
        } else {
            println!("NUMBER {} {}", number_str, number_str);
        }
    }
}
