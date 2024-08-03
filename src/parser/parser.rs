use crate::tokenizer::token::Token;


pub struct Parser {
    tokens: Vec<Token>,
    parsed: Vec<String>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens:tokens, parsed: vec![] }
    }

    pub fn parse_token(&mut self) {
        let len = self.tokens.len();
        
        for i in 0..len {
            match self.tokens[i].name.as_str() {
                "PLUS" => {
                    self.handle_plus(i);
                },
                "TRUE" | "FALSE" | "NIL" => {
                    self.parsed.push(self.tokens[i].symbol.to_string());
                },
                "NUMBER" => {
                    self.parsed.push(self.tokens[i].content.to_string());
                },
                "STRING" => {
                    self.parsed.push(self.tokens[i].content.to_string());
                },
                _ => {} // Ignore other tokens
            }
        }
    }

    fn handle_plus(&mut self, plus_index: usize) {
        let len = self.tokens.len();
        let left_number = "".to_string();
        let right_number = "".to_string();
        let left_is_number = if plus_index > 0 {
            self.tokens[plus_index - 1].name == "NUMBER"
        } else {
            false
        };
        
        let right_is_number = if plus_index < len - 1 {
            self.tokens[plus_index + 1].name == "NUMBER"
        } else {
            false
        };

        if left_is_number || right_is_number {
            self.process_plus_with_numbers(self.tokens[plus_index - 1].content.clone(), self.tokens[plus_index + 1].content.clone());
        }
    }

    fn process_plus_with_numbers(&mut self, left_number: String, right_number: String) {
        self.parsed.push(format!("(+  {} {})", {left_number},{right_number}));
    }

    pub fn print_parsed(&self) {
        for parsed_element in &self.parsed {
            println!("{}", parsed_element);
        }
    }
}
