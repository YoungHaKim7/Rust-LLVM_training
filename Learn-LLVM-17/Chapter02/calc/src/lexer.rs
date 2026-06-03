// Lexer module - tokenizes input expressions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Eoi,       // End of input
    Unknown,   // Unknown token
    Ident,     // Identifier
    Number,    // Number literal
    Comma,     // ,
    Colon,     // :
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    LParen,    // (
    RParen,    // )
    KwWith,    // 'with' keyword
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, text: String) -> Self {
        Self { kind, text }
    }

    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    pub fn is_one_of<const N: usize>(&self, kinds: [TokenKind; N]) -> bool {
        kinds.contains(&self.kind)
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.position;
        while let Some(c) = self.current_char() {
            if predicate(c) {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = match self.current_char() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eoi, String::new()),
        };

        if c.is_ascii_alphabetic() {
            let text = self.read_while(|c| c.is_ascii_alphabetic());
            let kind = if text == "with" {
                TokenKind::KwWith
            } else {
                TokenKind::Ident
            };
            Token::new(kind, text)
        } else if c.is_ascii_digit() {
            let text = self.read_while(|c| c.is_ascii_digit());
            Token::new(TokenKind::Number, text)
        } else {
            self.advance();
            let (kind, text) = match c {
                '+' => (TokenKind::Plus, "+".to_string()),
                '-' => (TokenKind::Minus, "-".to_string()),
                '*' => (TokenKind::Star, "*".to_string()),
                '/' => (TokenKind::Slash, "/".to_string()),
                '(' => (TokenKind::LParen, "(".to_string()),
                ')' => (TokenKind::RParen, ")".to_string()),
                ':' => (TokenKind::Colon, ":".to_string()),
                ',' => (TokenKind::Comma, ",".to_string()),
                _ => (TokenKind::Unknown, c.to_string()),
            };
            Token::new(kind, text)
        }
    }

    pub fn into_iter(self) -> LexerIterator {
        LexerIterator { lexer: self }
    }
}

pub struct LexerIterator {
    lexer: Lexer,
}

impl Iterator for LexerIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next_token();
        if token.kind == TokenKind::Eoi {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("123 456");
        assert_eq!(lexer.next_token().kind, TokenKind::Number);
        assert_eq!(lexer.next_token().kind, TokenKind::Number);
        assert_eq!(lexer.next_token().kind, TokenKind::Eoi);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * /");
        assert_eq!(lexer.next_token().kind, TokenKind::Plus);
        assert_eq!(lexer.next_token().kind, TokenKind::Minus);
        assert_eq!(lexer.next_token().kind, TokenKind::Star);
        assert_eq!(lexer.next_token().kind, TokenKind::Slash);
    }

    #[test]
    fn test_with_keyword() {
        let mut lexer = Lexer::new("with");
        assert_eq!(lexer.next_token().kind, TokenKind::KwWith);
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("abc xyz");
        assert_eq!(lexer.next_token().kind, TokenKind::Ident);
        assert_eq!(lexer.next_token().kind, TokenKind::Ident);
    }
}
