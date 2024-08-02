use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

struct Tokenizer {
    found_error: bool,
    line_number: usize,
    identifiers: HashSet<String>,
}

impl Tokenizer {
    fn new() -> Self {
        let mut identifiers = HashSet::new();
        identifiers.insert("and".to_string());
        identifiers.insert("class".to_string());
        identifiers.insert("else".to_string());
        identifiers.insert("false".to_string());
        identifiers.insert("for".to_string());
        identifiers.insert("fun".to_string());
        identifiers.insert("if".to_string());
        identifiers.insert("nil".to_string());
        identifiers.insert("or".to_string());
        identifiers.insert("print".to_string());
        identifiers.insert("return".to_string());
        identifiers.insert("super".to_string());
        identifiers.insert("this".to_string());
        identifiers.insert("true".to_string());
        identifiers.insert("var".to_string());
        identifiers.insert("while".to_string());
        Self {
            found_error: false,
            line_number: 1,
            identifiers,
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
                        //Comment detected, skip line
                        break;
                    };

                },
                '\t' | ' ' => {}, // Ignore tabs and spaces
                '"' => self.handle_string(&chars, &mut i),
                '0'..='9' => self.handle_number(&chars, &mut i),
                'a'..='z' | 'A'..='Z' | '_' => self.handle_identifier(&chars, &mut i),
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
            // Comment detected, skip the rest of the line
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
                if (*i + 1) < chars.len() && chars[*i + 1].is_digit(10){
                    decimal_found = true;
                }else {
                    break;
                }
            }
            number_str.push(chars[*i]);
            *i += 1;
        }

        // Decrease `i` by 1 to counter the extra increment in the loop
        *i -= 1;

        let mut interpreted_number = number_str.clone();
        // 200.100 -> 200.1y
        if decimal_found {
            interpreted_number = interpreted_number.trim_end_matches('0').to_string();
        }
        // 200 -> 200.0
        if !interpreted_number.contains('.') {
            interpreted_number = format!("{}.0", interpreted_number);
        }
        // 200. -> 200.0
        if interpreted_number.ends_with("."){
            interpreted_number = format!("{}0", interpreted_number);
        }

        println!("NUMBER {} {}", number_str, interpreted_number);
 
    }

    fn handle_identifier(&mut self, chars: &[char], i: &mut usize) {
        let mut identifier_str = String::new();

        // Collect identifier characters
        while *i < chars.len() && (chars[*i].is_alphanumeric() || chars[*i] == '_') {
            identifier_str.push(chars[*i]);
            *i += 1;
        }

        // Adjust `i` by 1 to counter the extra increment in the loop
        *i -= 1;


        // Accept all identifiers
        

        if self.identifiers.contains(&identifier_str) {
            println!("{} {} null",identifier_str.to_uppercase(), identifier_str.to_lowercase());
        } else {
            // Handle undefined identifiers if necessary
            println!("IDENTIFIER {} null", identifier_str);
        }
    }

    fn handle_unexpected(&mut self, c: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", self.line_number, c);
        self.found_error = true;
    }
}

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
