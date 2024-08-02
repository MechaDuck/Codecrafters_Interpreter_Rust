use std::collections::HashSet;

use super::token::Token;

pub struct Tokenizer {
    pub found_error: bool,
    pub line_number: u64,
    pub tokens: Vec<Token>,
    identifiers: HashSet<String>,
}

impl Tokenizer {
    pub fn new() -> Self {
        let identifiers = vec![
            "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print",
            "return", "super", "this", "true", "var", "while",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            found_error: false,
            line_number: 1,
            tokens: Vec::new(),
            identifiers,
        }
    }

    pub fn tokenize(&mut self, line: &str) {
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        let mut char_number = 0;

        while i < chars.len() {
            let start_char_number = char_number;
            match chars[i] {
                '(' => self.add_token("LEFT_PAREN", "(", "", start_char_number, start_char_number + 1),
                ')' => self.add_token("RIGHT_PAREN", ")", "", start_char_number, start_char_number + 1),
                '{' => self.add_token("LEFT_BRACE", "{", "", start_char_number, start_char_number + 1),
                '}' => self.add_token("RIGHT_BRACE", "}", "", start_char_number, start_char_number + 1),
                '*' => self.add_token("STAR", "*", "", start_char_number, start_char_number + 1),
                ',' => self.add_token("COMMA", ",", "", start_char_number, start_char_number + 1),
                '.' => self.add_token("DOT", ".", "", start_char_number, start_char_number + 1),
                '+' => self.add_token("PLUS", "+", "", start_char_number, start_char_number + 1),
                '-' => self.add_token("MINUS", "-", "", start_char_number, start_char_number + 1),
                ';' => self.add_token("SEMICOLON", ";", "", start_char_number, start_char_number + 1),
                '=' => self.handle_equal(&chars, &mut i, start_char_number),
                '!' => self.handle_bang(&chars, &mut i, start_char_number),
                '<' => self.handle_less(&chars, &mut i, start_char_number),
                '>' => self.handle_greater(&chars, &mut i, start_char_number),
                '/' => {
                    if self.handle_slash(&chars, &mut i, start_char_number) {
                        break; // Comment detected, skip line
                    }
                },
                '\t' | ' ' => {}, // Ignore tabs and spaces
                '"' => self.handle_string(&chars, &mut i, start_char_number),
                '0'..='9' => self.handle_number(&chars, &mut i, start_char_number),
                'a'..='z' | 'A'..='Z' | '_' => self.handle_identifier(&chars, &mut i, start_char_number),
                _ => self.handle_unexpected(chars[i], start_char_number),
            }
            i += 1;
            char_number = start_char_number + 1;
        }
        self.add_token("EOF", "", "null",0, 0)
    }

    fn add_token(&mut self, token_type: &str, value: &str, content: &str, start_char_number: u64, end_char_number: u64) {
        let token = Token::new(
            token_type.to_string(),
            value.to_string(),
            content.to_string(),
            self.line_number,
            start_char_number,
            end_char_number,
            None,
        );
        self.tokens.push(token);
    }

    fn handle_equal(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            self.add_token("EQUAL_EQUAL", "==", "", start_char_number, start_char_number + 2);
            *i += 1; // Skip the next character
        } else {
            self.add_token("EQUAL", "=", "", start_char_number, start_char_number + 1);
        }
    }

    fn handle_bang(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            self.add_token("BANG_EQUAL", "!=", "", start_char_number, start_char_number + 2);
            *i += 1; // Skip the next character
        } else {
            self.add_token("BANG", "!", "", start_char_number, start_char_number + 1);
        }
    }

    fn handle_less(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            self.add_token("LESS_EQUAL", "<=", "", start_char_number, start_char_number + 2);
            *i += 1; // Skip the next character
        } else {
            self.add_token("LESS", "<", "", start_char_number, start_char_number + 1);
        }
    }

    fn handle_greater(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) {
        if *i + 1 < chars.len() && chars[*i + 1] == '=' {
            self.add_token("GREATER_EQUAL", ">=", "", start_char_number, start_char_number + 2);
            *i += 1; // Skip the next character
        } else {
            self.add_token("GREATER", ">", "", start_char_number, start_char_number + 1);
        }
    }

    fn handle_slash(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) -> bool {
        if *i + 1 < chars.len() && chars[*i + 1] == '/' {
            return true; // Comment detected, skip the rest of the line
        }
        self.add_token("SLASH", "/", "", start_char_number, start_char_number + 1);
        false
    }

    fn handle_string(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) {
        let mut tmp_string = String::new();
        let mut found_string_end = false;
        *i += 1; // Skip the starting quote
        let mut end_char_number = start_char_number + 1;

        while *i < chars.len() {
            if chars[*i] == '"' {
                found_string_end = true;
                end_char_number += 1;
                break;
            }
            tmp_string.push(chars[*i]);
            *i += 1;
            end_char_number += 1;
        }

        if !found_string_end {
            let error_message = format!("[line {}] Error: Unterminated string.", self.line_number);
            let token = Token::new(
                "ERROR".to_string(),
                "STRING".to_string(),
                tmp_string,
                self.line_number,
                start_char_number,
                end_char_number,
                Some(error_message.clone()),
            );
            self.tokens.push(token);
            self.found_error = true;
        } else {
            self.add_token("STRING", &tmp_string, &tmp_string, start_char_number, end_char_number);
        }
    }

    fn handle_number(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) {
        let mut number_str = String::new();
        let mut decimal_found = false;
        let mut end_char_number = start_char_number;

        while *i < chars.len() && (chars[*i].is_digit(10) || chars[*i] == '.') {
            if chars[*i] == '.' {
                if decimal_found {
                    let error_message = format!("[line {}] Error: Multiple decimal points in number.", self.line_number);
                    let token = Token::new(
                        "ERROR".to_string(),
                        "NUMBER".to_string(),
                        number_str.clone(),
                        self.line_number,
                        start_char_number,
                        end_char_number + 1,
                        Some(error_message.clone()),
                    );
                    self.tokens.push(token);
                    self.found_error = true;
                    return;
                }
                decimal_found = true;
            }
            number_str.push(chars[*i]);
            *i += 1;
            end_char_number += 1;
        }

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

        self.add_token("NUMBER", &number_str, &interpreted_number, start_char_number, end_char_number);
    }

    fn handle_identifier(&mut self, chars: &[char], i: &mut usize, start_char_number: u64) {
        let mut identifier_str = String::new();
        let mut end_char_number = start_char_number;

        while *i < chars.len() && (chars[*i].is_alphanumeric() || chars[*i] == '_') {
            identifier_str.push(chars[*i]);
            *i += 1;
            end_char_number += 1;
        }

        *i -= 1;

        if self.identifiers.contains(&identifier_str) {
            self.add_token(&identifier_str.to_uppercase(), &identifier_str, "", start_char_number, end_char_number);
        } else {
            self.add_token("IDENTIFIER", &identifier_str, "", start_char_number, end_char_number);
        }
    }

    fn handle_unexpected(&mut self, c: char, start_char_number: u64) {
        let error_message = format!("[line {}] Error: Unexpected character: {}", self.line_number, c);
        let token = Token::new(
            "ERROR".to_string(),
            c.to_string(),
            "".to_string(),
            self.line_number,
            start_char_number,
            start_char_number + 1,
            Some(error_message.clone()),
        );
        self.tokens.push(token);
        self.found_error = true;
    }
}