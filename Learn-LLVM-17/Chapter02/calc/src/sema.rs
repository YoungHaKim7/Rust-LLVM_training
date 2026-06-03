// Semantic Analysis module - checks variable declarations

use crate::ast::{Ast, Expr, Factor};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub enum SemanticError {
    VariableAlreadyDeclared(String),
    VariableNotDeclared(String),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::VariableAlreadyDeclared(name) => {
                write!(f, "Variable {} already declared", name)
            }
            SemanticError::VariableNotDeclared(name) => {
                write!(f, "Variable {} not declared", name)
            }
        }
    }
}

pub type SemanticResult<T> = Result<T, Vec<SemanticError>>;

pub struct Sema;

impl Sema {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, ast: &Ast) -> SemanticResult<()> {
        let mut errors = Vec::new();
        let mut scope = HashSet::new();

        match ast {
            Ast::Expr(expr) => {
                Self::analyze_expr(expr, &mut scope, &mut errors);
            }
            Ast::WithDecl(decl) => {
                // Add variables to scope
                for var in &decl.vars {
                    if !scope.insert(var.clone()) {
                        errors.push(SemanticError::VariableAlreadyDeclared(var.clone()));
                    }
                }
                // Analyze the expression
                Self::analyze_expr(&decl.expr, &mut scope, &mut errors);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn analyze_expr(expr: &Expr, scope: &mut HashSet<String>, errors: &mut Vec<SemanticError>) {
        match expr {
            Expr::Factor(factor) => Self::analyze_factor(factor, scope, errors),
            Expr::BinaryOp(op) => {
                Self::analyze_expr(&op.left(), scope, errors);
                Self::analyze_expr(&op.right(), scope, errors);
            }
        }
    }

    fn analyze_factor(factor: &Factor, scope: &HashSet<String>, errors: &mut Vec<SemanticError>) {
        if let Factor::Ident(name) = factor {
            if !scope.contains(name) {
                errors.push(SemanticError::VariableNotDeclared(name.clone()));
            }
        }
        // Number factors don't need semantic analysis
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_valid_variable_use() {
        let input = "with x: x + 1";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap().unwrap();

        let sema = Sema::new();
        let result = sema.analyze(&ast);
        assert!(result.is_ok());
    }

    #[test]
    fn test_undeclared_variable() {
        let input = "x + 1";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap().unwrap();

        let sema = Sema::new();
        let result = sema.analyze(&ast);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_variable() {
        let input = "with x, x: x + 1";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap().unwrap();

        let sema = Sema::new();
        let result = sema.analyze(&ast);
        assert!(result.is_err());
    }

    #[test]
    fn test_number_only() {
        let input = "1 + 2 * 3";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap().unwrap();

        let sema = Sema::new();
        let result = sema.analyze(&ast);
        assert!(result.is_ok());
    }
}
