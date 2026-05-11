//! Definición del AST (Abstract Syntax Tree) para expresiones matemáticas.
//! Implementación pendiente para Fase 1.

/// Nodo del AST que representa una expresión matemática.
#[derive(Debug, Clone)]
pub enum AstNode {
    Number(f64),
    BinaryOp {
        op: BinaryOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Variable(String),
    Constant(Constant),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Pos,
}

#[derive(Debug, Clone)]
pub enum Constant {
    Pi,
    E,
}
