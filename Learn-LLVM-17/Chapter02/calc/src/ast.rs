// AST definitions using Rust enums instead of inheritance

use std::fmt;

/// Represents all possible AST nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Expr(Expr),
    WithDecl(WithDecl),
}

/// Expression nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Factor(Factor),
    BinaryOp(BinaryOp),
}

/// Factor - either an identifier or a number
#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Ident(String),
    Number(i32),
}

/// Binary operations
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add { left: Box<Expr>, right: Box<Expr> },
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
    Div { left: Box<Expr>, right: Box<Expr> },
}

impl BinaryOp {
    pub fn left(&self) -> &Expr {
        match self {
            BinaryOp::Add { left, .. } => left,
            BinaryOp::Sub { left, .. } => left,
            BinaryOp::Mul { left, .. } => left,
            BinaryOp::Div { left, .. } => left,
        }
    }

    pub fn right(&self) -> &Expr {
        match self {
            BinaryOp::Add { right, .. } => right,
            BinaryOp::Sub { right, .. } => right,
            BinaryOp::Mul { right, .. } => right,
            BinaryOp::Div { right, .. } => right,
        }
    }
}

/// "with" declaration for variable bindings
#[derive(Debug, Clone, PartialEq)]
pub struct WithDecl {
    pub vars: Vec<String>,
    pub expr: Expr,
}

impl WithDecl {
    pub fn new(vars: Vec<String>, expr: Expr) -> Self {
        Self { vars, expr }
    }

    pub fn iter_vars(&self) -> impl Iterator<Item = &String> {
        self.vars.iter()
    }
}

impl From<Factor> for Expr {
    fn from(factor: Factor) -> Self {
        Expr::Factor(factor)
    }
}

impl From<BinaryOp> for Expr {
    fn from(op: BinaryOp) -> Self {
        Expr::BinaryOp(op)
    }
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Factor::Ident(name) => write!(f, "{}", name),
            Factor::Number(n) => write!(f, "{}", n),
        }
    }
}
