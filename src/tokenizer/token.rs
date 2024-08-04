pub struct Token {
    pub name: String,
    pub symbol: String,
    pub content: String,
    line: u64,
    start_char_number: u64,
    end_char_number: u64,
    error: Option<String>,
}

impl Token {
    pub fn new(name: String, symbol: String, content: String, line: u64, start_char_number: u64, end_char_number: u64, error: Option<String>) -> Self {
        Self { name, symbol, content, line, start_char_number, end_char_number, error }
    }

    pub fn print(&self) {
        if let Some(ref err) = self.error {
            eprintln!("[line {}] Error: {}", self.line, err);
        } else {
            println!("{} {} {}", self.name, self.symbol, self.content);
        }
    }

    pub fn print_advanced_error(&self) {
        if let Some(ref err) = self.error {
            eprintln!("[line {}:{}-{}] Error: {}", self.line, self.start_char_number, self.end_char_number, err);
        } else {
            println!("{} {} {}", self.name, self.symbol, self.content);
        }
    }
}
