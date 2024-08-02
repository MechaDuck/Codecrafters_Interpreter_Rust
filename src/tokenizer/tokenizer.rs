use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

pub struct Tokenizer {
    pub found_error: bool,
    pub line_number: usize,
    identifiers: HashSet<String>,
}

impl Tokenizer {
    // Initialize a new Tokenizer with a predefined set of identifiers.
    pub fn new() -> Self {
        let identifiers = vec![
            "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print",
            "return", "super", "this", "true", "var", "while"
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            found_error: false,
            line_number: 1,
            identifiers,
        }
    }

    // Tokenize a single line of text.
    pub fn tokenize(&mut self, line: &str) {
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
                    if self.handle_slash(&chars, &mut i) {
                        break; // Comment detected, skip line
                    }
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

    // Print a token with its type and value.
    fn print_token(&self, token_type: &str, value: &str) {
        println!("{} {} null", token_type, value);
    }

    // Handle '=' token, including '=='.
    fn handle_equal(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("EQUAL_EQUAL == null");
            *i += 1; // Skip the next character
        } else {
            println!("EQUAL = null");
        }
    }

    // Handle '!' token, including '!='.
    fn handle_bang(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("BANG_EQUAL != null");
            *i += 1; // Skip the next character
        } else {
            println!("BANG ! null");
        }
    }

    // Handle '<' token, including '<='.
    fn handle_less(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("LESS_EQUAL <= null");
            *i += 1; // Skip the next character
        } else {
            println!("LESS < null");
        }
    }

    // Handle '>' token, including '>='.
    fn handle_greater(&mut self, chars: &[char], i: &mut usize) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            println!("GREATER_EQUAL >= null");
            *i += 1; // Skip the next character
        } else {
            println!("GREATER > null");
        }
    }

    // Handle '/' token, including comments.
    fn handle_slash(&mut self, chars: &[char], i: &mut usize) -> bool {
        if *i + 1 < chars.len() && chars[*i + 1] == '/' {
            return true; // Comment detected, skip the rest of the line
        }
        println!("SLASH / null");
        false
    }

    // Handle string literals enclosed in double quotes.
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
                tmp_string
            );
        }
    }

    // Handle numeric literals, including formatting floats.
    fn handle_number(&mut self, chars: &[char], i: &mut usize) {
        let mut number_str = String::new();
        let mut decimal_found = false;

        while *i < chars.len() && (chars[*i].is_digit(10) || chars[*i] == '.') {
            if chars[*i] == '.' {
                if decimal_found {
                    // Error handling: multiple decimal points
                    eprintln!("[line {}] Error: Multiple decimal points in number.", self.line_number);
                    self.found_error = true;
                    break;
                }
                decimal_found = true;
            }
            number_str.push(chars[*i]);
            *i += 1;
        }

        // Adjust `i` by 1 to counter the extra increment in the loop
        *i -= 1;

        // Format the number
        let mut interpreted_number = number_str.clone();
        if decimal_found {
            interpreted_number = interpreted_number.trim_end_matches('0').to_string();
            if interpreted_number.ends_with('.') {
                interpreted_number.push('0');
            }
        } else {
            interpreted_number = format!("{}.0", number_str);
        }

        println!("NUMBER {} {}", number_str, interpreted_number);
    }

    // Handle identifiers, including predefined ones.
    fn handle_identifier(&mut self, chars: &[char], i: &mut usize) {
        let mut identifier_str = String::new();

        while *i < chars.len() && (chars[*i].is_alphanumeric() || chars[*i] == '_') {
            identifier_str.push(chars[*i]);
            *i += 1;
        }

        // Adjust `i` by 1 to counter the extra increment in the loop
        *i -= 1;

        if self.identifiers.contains(&identifier_str) {
            println!("{} {} null", identifier_str.to_uppercase(), identifier_str);
        } else {
            println!("IDENTIFIER {} null", identifier_str);
        }
    }

    // Handle unexpected characters.
    fn handle_unexpected(&mut self, c: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", self.line_number, c);
        self.found_error = true;
    }
}
