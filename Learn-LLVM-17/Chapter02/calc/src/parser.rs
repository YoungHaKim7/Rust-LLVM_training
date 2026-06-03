// Parser module - constructs AST from tokens

use crate::ast::{Ast, BinaryOp, Expr, Factor, WithDecl};
use crate::lexer::{Lexer, Token, TokenKind};
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: TokenKind, found: Token },
    UnexpectedEoi,
    InvalidExpression,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Expected {:?}, found {:?}", expected, found.kind)
            }
            ParseError::UnexpectedEoi => write!(f, "Unexpected end of input"),
            ParseError::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current: Token,
    has_error: bool,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Self {
            lexer,
            current,
            has_error: false,
        }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.current.kind != kind {
            Err(ParseError::UnexpectedToken {
                expected: kind,
                found: self.current.clone(),
            })
        } else {
            Ok(())
        }
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        self.expect(kind)?;
        self.advance();
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Option<Ast>, ParseError> {
        let result = self.parse_calc()?;
        self.expect(TokenKind::Eoi)?;
        Ok(result)
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    fn parse_calc(&mut self) -> Result<Option<Ast>, ParseError> {
        let mut vars = Vec::new();

        if self.current.kind == TokenKind::KwWith {
            self.advance();

            self.expect(TokenKind::Ident)?;
            vars.push(self.current.text.clone());
            self.advance();

            while self.current.kind == TokenKind::Comma {
                self.advance();
                self.expect(TokenKind::Ident)?;
                vars.push(self.current.text.clone());
                self.advance();
            }

            self.consume(TokenKind::Colon)?;
        }

        let expr = self.parse_expr()?;

        if vars.is_empty() {
            Ok(expr.map(|e| Ast::Expr(e)))
        } else {
            let expr = expr.ok_or(ParseError::InvalidExpression)?;
            Ok(Some(Ast::WithDecl(WithDecl::new(vars, expr))))
        }
    }

    fn parse_expr(&mut self) -> Result<Option<Expr>, ParseError> {
        let mut left = self.parse_term()?;

        while self.current.is_one_of([TokenKind::Plus, TokenKind::Minus]) {
            let is_plus = self.current.kind == TokenKind::Plus;
            self.advance(); // Consume the operator
            let right = self.parse_term()?;

            let op = if is_plus {
                BinaryOp::Add {
                    left: Box::new(left.unwrap()),
                    right: Box::new(right.unwrap()),
                }
            } else {
                BinaryOp::Sub {
                    left: Box::new(left.unwrap()),
                    right: Box::new(right.unwrap()),
                }
            };

            left = Some(Expr::BinaryOp(op));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Option<Expr>, ParseError> {
        let mut left = self.parse_factor()?;

        while self.current.is_one_of([TokenKind::Star, TokenKind::Slash]) {
            let is_star = self.current.kind == TokenKind::Star;
            self.advance(); // Consume the operator
            let right = self.parse_factor()?;

            let op = if is_star {
                BinaryOp::Mul {
                    left: Box::new(left.clone().unwrap()),
                    right: Box::new(right.unwrap()),
                }
            } else {
                BinaryOp::Div {
                    left: Box::new(left.clone().unwrap()),
                    right: Box::new(right.unwrap()),
                }
            };

            left = Some(Expr::BinaryOp(op));
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Option<Expr>, ParseError> {
        match self.current.kind {
            TokenKind::Number => {
                let value = self.current.text.parse::<i32>().unwrap_or(0);
                let factor = Factor::Number(value);
                self.advance();
                Ok(Some(Expr::Factor(factor)))
            }
            TokenKind::Ident => {
                let name = self.current.text.clone();
                let factor = Factor::Ident(name);
                self.advance();
                Ok(Some(Expr::Factor(factor)))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.consume(TokenKind::RParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Number,
                found: self.current.clone(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_number() {
        let lexer = Lexer::new("42");
        let mut parser = Parser::new(lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_simple_arithmetic() {
        let lexer = Lexer::new("1 + 2 * 3");
        let mut parser = Parser::new(lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_with_declaration() {
        let lexer = Lexer::new("with x, y: x + y");
        let mut parser = Parser::new(lexer);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
