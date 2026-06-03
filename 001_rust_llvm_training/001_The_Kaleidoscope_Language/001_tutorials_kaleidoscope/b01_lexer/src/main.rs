use std::io::{self, Read, Write};

// Token type - using Option to distinguish between tokens and ASCII values
#[derive(Debug, PartialEq, Clone)]
enum Token {
    // Commands
    Def,
    Extern,

    // Primary
    Identifier(String),
    Number(f64),

    // Control flow
    Eof,
}

// Lexer state structure
struct Lexer {
    last_char: Option<char>,
    identifier_str: String,
    num_val: f64,
}

impl Lexer {
    fn new() -> Self {
        Lexer {
            last_char: None,
            identifier_str: String::new(),
            num_val: 0.0,
        }
    }

    fn get_char(&mut self) -> Option<char> {
        if let Some(c) = self.last_char.take() {
            return Some(c);
        }

        let mut buffer = [0u8; 1];
        match io::stdin().read_exact(&mut buffer) {
            Ok(_) => Some(buffer[0] as char),
            Err(_) => None,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.last_char.is_none() {
            self.last_char = self.get_char();
        }
        self.last_char
    }

    fn is_space(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\r' || c == '\n'
    }

    fn is_alnum(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    // Get the current identifier string (for compatibility with tutorial)
    fn get_identifier_str(&self) -> &str {
        &self.identifier_str
    }

    // Get the current numeric value (for compatibility with tutorial)
    fn get_num_val(&self) -> f64 {
        self.num_val
    }

    // The main lexer function - returns Token for recognized tokens,
    // or ASCII value for unknown characters
    fn gettok(&mut self) -> Result<Token, i32> {
        // Skip any whitespace
        while let Some(c) = self.get_char() {
            if !Self::is_space(c) {
                self.last_char = Some(c);
                break;
            }
        }

        // Check for end of file
        let current_char = match self.get_char() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        // Identifier: [a-zA-Z][a-zA-Z0-9]*
        if current_char.is_alphabetic() || current_char == '_' {
            self.identifier_str.clear();
            self.identifier_str.push(current_char);

            while let Some(c) = self.peek_char() {
                if Self::is_alnum(c) {
                    let c = self.get_char().unwrap();
                    self.identifier_str.push(c);
                } else {
                    break;
                }
            }

            // Check for known keywords
            if self.identifier_str == "def" {
                return Ok(Token::Def);
            }
            if self.identifier_str == "extern" {
                return Ok(Token::Extern);
            }

            return Ok(Token::Identifier(self.identifier_str.clone()));
        }

        // Number: [0-9.]+
        if current_char.is_ascii_digit() || current_char == '.' {
            let mut num_str = String::new();
            num_str.push(current_char);

            while let Some(c) = self.peek_char() {
                if c.is_ascii_digit() || c == '.' {
                    let c = self.get_char().unwrap();
                    num_str.push(c);
                } else {
                    break;
                }
            }

            // Parse the number
            match num_str.parse::<f64>() {
                Ok(num) => {
                    self.num_val = num;
                    return Ok(Token::Number(num));
                }
                Err(_) => {
                    eprintln!("Invalid number format: {}", num_str);
                    return Err(current_char as i32);
                }
            }
        }

        // Return ASCII value for unknown characters
        Err(current_char as i32)
    }
}

fn main() {
    let mut lexer = Lexer::new();

    println!("Kaleidoscope Lexer");
    println!("Enter expressions (Ctrl+D to exit):");

    loop {
        print!("ready> ");
        io::stdout().flush().unwrap();

        match lexer.gettok() {
            Ok(token) => {
                match token {
                    Token::Def => println!("Token: Def"),
                    Token::Extern => println!("Token: Extern"),
                    Token::Identifier(_name) => {
                        println!("Token: Identifier({})", lexer.get_identifier_str());
                    }
                    Token::Number(_num) => {
                        println!("Token: Number({})", lexer.get_num_val());
                    }
                    Token::Eof => {
                        println!("Token: Eof");
                        break;
                    }
                }
            }
            Err(ascii_val) => {
                if ascii_val == 4 {
                    // Ctrl+D
                    println!("Token: Eof");
                    break;
                }
                println!("Token: ASCII('{}')", ascii_val as u8 as char);
            }
        }
    }
}
