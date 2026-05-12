//! Definición del AST (Abstract Syntax Tree) y algoritmo Shunting Yard
//! para convertir tokens infijos a notación polaca inversa (RPN).
//!
//! El shunting yard produce un `Vec<AstNode>` en orden RPN. Los nodos de
//! operadores llevan operandos dummy (`Number(0.0)`) ya que en la evaluación
//! RPN los operandos provienen de la pila de valores.
//!
//! ## Precedencia de operadores
//!
//! | Nivel | Operadores              | Asociatividad |
//! |-------|-------------------------|---------------|
//! | 1     | Bitwise OR              | Izquierda     |
//! | 2     | Bitwise XOR             | Izquierda     |
//! | 3     | Bitwise AND             | Izquierda     |
//! | 4     | <<, >>                  | Izquierda     |
//! | 5     | +, -                    | Izquierda     |
//! | 6     | *, /, %                 | Izquierda     |
//! | 7     | ^                       | Derecha       |
//! | 8     | Funciones               | —             |
//! | 9     | - unario, ~, ! factorial| —             |

use crate::models::errors::{CalcError, ErrorKind};
use crate::parser::tokenizer::Token;

// ─── AST ───────────────────────────────────────────────────────────────────

/// Nodo del AST que representa una expresión matemática en formato RPN.
#[derive(Debug, Clone)]
pub enum AstNode {
    Number(f64),
    UnaryOp {
        op: UnaryOperator,
        /// En RPN el operando es dummy; la evaluación usa la pila.
        operand: Box<AstNode>,
    },
    BinaryOp {
        op: BinaryOperator,
        /// En RPN los operandos son dummy; la evaluación usa la pila.
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Function {
        name: String,
        /// Número de argumentos que la función espera en la pila.
        /// Para funciones de un argumento (todas las actuales), es 1.
        /// args queda vacío en representación RPN plana.
        args: Vec<AstNode>,
    },
    Constant(String),
}

/// Operadores unarios.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Factorial,
    BitNot,
}

/// Operadores binarios.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

// ─── Precedencia ───────────────────────────────────────────────────────────

/// Nivel de precedencia de un operador binario (mayor = más prioritario).
fn binary_precedence(op: &BinaryOperator) -> u8 {
    match op {
        BinaryOperator::BitOr => 1,
        BinaryOperator::BitXor => 2,
        BinaryOperator::BitAnd => 3,
        BinaryOperator::Shl | BinaryOperator::Shr => 4,
        BinaryOperator::Add | BinaryOperator::Sub => 5,
        BinaryOperator::Mul | BinaryOperator::Div | BinaryOperator::Mod => 6,
        BinaryOperator::Pow => 7,
    }
}

/// Indica si un operador binario es asociativo por la derecha.
fn is_right_associative(op: &BinaryOperator) -> bool {
    matches!(op, BinaryOperator::Pow)
}

// ─── Helper ────────────────────────────────────────────────────────────────

fn dummy_number() -> Box<AstNode> {
    Box::new(AstNode::Number(0.0))
}

// ─── Tipos internos de la pila ─────────────────────────────────────────────

/// Elemento en la pila de operadores del algoritmo shunting yard.
#[derive(Debug, Clone)]
enum StackItem {
    /// Operador binario.
    BinOp(BinaryOperator),
    /// Operador unario (Negate).
    UnaryOp(UnaryOperator),
    /// Marcador de función con su nombre y número de argumentos acumulados.
    Function { name: String, arg_count: usize },
    /// Paréntesis izquierdo.
    LParen,
}

// ─── Shunting Yard ─────────────────────────────────────────────────────────

/// Convierte una secuencia de tokens infijos a RPN como `Vec<AstNode>`.
///
/// # Algoritmo
///
/// Recorre los tokens de izquierda a derecha:
/// - Números y constantes van directamente a la salida.
/// - Funciones se apilan.
/// - Operadores se apilan respetando precedencia y asociatividad.
/// - `(` se apila.
/// - `)` desapila hasta encontrar `(`; si hay una función justo debajo, se emite.
/// - `,` desapila hasta `(` e incrementa el contador de argumentos de la función.
/// - Al final, se vacía la pila a la salida.
pub fn shunting_yard(tokens: &[Token]) -> Result<Vec<AstNode>, CalcError> {
    let mut output: Vec<AstNode> = Vec::new();
    let mut op_stack: Vec<StackItem> = Vec::new();
    let mut paren_balance: i32 = 0;

    /// Determina si un token `Minus` en la posición dada debe interpretarse
    /// como negación unaria (en vez de resta binaria).
    fn is_unary_context(idx: usize, tokens: &[Token]) -> bool {
        if idx == 0 {
            return true;
        }
        match &tokens[idx - 1] {
            Token::Number(_)
            | Token::RParen
            | Token::Factorial
            | Token::Pi
            | Token::E
            | Token::Ans => false,
            _ => true,
        }
    }

    /// Emite un `StackItem` como `AstNode` al vector de salida.
    fn emit(item: StackItem, output: &mut Vec<AstNode>) {
        match item {
            StackItem::BinOp(op) => {
                output.push(AstNode::BinaryOp {
                    op,
                    left: dummy_number(),
                    right: dummy_number(),
                });
            }
            StackItem::UnaryOp(op) => {
                output.push(AstNode::UnaryOp {
                    op,
                    operand: dummy_number(),
                });
            }
            StackItem::Function { name, arg_count: _ } => {
                output.push(AstNode::Function {
                    name,
                    args: Vec::new(),
                });
            }
            StackItem::LParen => {
                // Los paréntesis no se emiten; se descartan al desapilar.
            }
        }
    }

    /// Decide si el operador en la cima de la pila debe desapilarse antes de
    /// apilar `current`.
    fn should_pop_stack(
        top: &StackItem,
        current: &BinaryOperator,
    ) -> bool {
        match top {
            StackItem::BinOp(top_op) => {
                let tp = binary_precedence(top_op);
                let cp = binary_precedence(current);
                // Desapila si la cima tiene mayor precedencia, o igual
                // precedencia y el operador actual es asociativo izquierdo.
                tp > cp || (tp == cp && !is_right_associative(current))
            }
            // Unario y función siempre tienen mayor precedencia que binario.
            StackItem::UnaryOp(_) | StackItem::Function { .. } => true,
            StackItem::LParen => false,
        }
    }

    let len = tokens.len();
    let mut idx = 0;

    while idx < len {
        let token = &tokens[idx];

        match token {
            // ── Valores ───────────────────────────────────────────────
            Token::Number(n) => {
                output.push(AstNode::Number(*n));
            }
            Token::Pi => {
                output.push(AstNode::Constant("pi".to_string()));
            }
            Token::E => {
                output.push(AstNode::Constant("e".to_string()));
            }
            Token::Ans => {
                output.push(AstNode::Constant("ans".to_string()));
            }

            // ── Funciones ──────────────────────────────────────────────
            Token::Sin => op_stack.push(StackItem::Function { name: "sin".into(), arg_count: 1 }),
            Token::Cos => op_stack.push(StackItem::Function { name: "cos".into(), arg_count: 1 }),
            Token::Tan => op_stack.push(StackItem::Function { name: "tan".into(), arg_count: 1 }),
            Token::Asin => op_stack.push(StackItem::Function { name: "asin".into(), arg_count: 1 }),
            Token::Acos => op_stack.push(StackItem::Function { name: "acos".into(), arg_count: 1 }),
            Token::Atan => op_stack.push(StackItem::Function { name: "atan".into(), arg_count: 1 }),
            Token::Sinh => op_stack.push(StackItem::Function { name: "sinh".into(), arg_count: 1 }),
            Token::Cosh => op_stack.push(StackItem::Function { name: "cosh".into(), arg_count: 1 }),
            Token::Tanh => op_stack.push(StackItem::Function { name: "tanh".into(), arg_count: 1 }),
            Token::Log => op_stack.push(StackItem::Function { name: "log".into(), arg_count: 1 }),
            Token::Ln => op_stack.push(StackItem::Function { name: "ln".into(), arg_count: 1 }),
            Token::Log2 => op_stack.push(StackItem::Function { name: "log2".into(), arg_count: 1 }),
            Token::Log10 => op_stack.push(StackItem::Function { name: "log10".into(), arg_count: 1 }),
            Token::Sqrt => op_stack.push(StackItem::Function { name: "sqrt".into(), arg_count: 1 }),
            Token::Cbrt => op_stack.push(StackItem::Function { name: "cbrt".into(), arg_count: 1 }),
            Token::Abs => op_stack.push(StackItem::Function { name: "abs".into(), arg_count: 1 }),
            Token::Exp => op_stack.push(StackItem::Function { name: "exp".into(), arg_count: 1 }),

            // ── Coma (separador de argumentos) ──────────────────────────
            Token::Comma => {
                // Desapilar operadores hasta encontrar LParen.
                loop {
                    match op_stack.pop() {
                        Some(item @ StackItem::LParen) => {
                            // Buscar la función justo debajo del LParen e
                            // incrementar su contador de argumentos.
                            match op_stack.pop() {
                                Some(StackItem::Function { name, arg_count }) => {
                                    op_stack.push(StackItem::Function {
                                        name,
                                        arg_count: arg_count + 1,
                                    });
                                }
                                other => {
                                    // No hay función: coma en contexto no válido.
                                    // Devolver lo que sacamos.
                                    if let Some(item) = other {
                                        op_stack.push(item);
                                    }
                                    return Err(CalcError::new(
                                        ErrorKind::ParseError,
                                        "Coma fuera de una llamada a función".to_string(),
                                    ));
                                }
                            }
                            op_stack.push(item); // reponer LParen
                            break;
                        }
                        Some(other) => emit(other, &mut output),
                        None => {
                            return Err(CalcError::new(
                                ErrorKind::ParseError,
                                "Coma inesperada fuera de función".to_string(),
                            ));
                        }
                    }
                }
            }

            // ── Paréntesis izquierdo ────────────────────────────────────
            Token::LParen => {
                op_stack.push(StackItem::LParen);
                paren_balance += 1;
            }

            // ── Paréntesis derecho ──────────────────────────────────────
            Token::RParen => {
                paren_balance -= 1;
                if paren_balance < 0 {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        "Paréntesis derecho sin correspondencia".to_string(),
                    ));
                }

                // Desapilar hasta encontrar LParen.
                loop {
                    match op_stack.pop() {
                        Some(StackItem::LParen) => {
                            // Si justo debajo del LParen hay una función, emitirla.
                            if matches!(op_stack.last(), Some(StackItem::Function { .. })) {
                                if let Some(StackItem::Function { name, .. }) =
                                    op_stack.pop()
                                {
                                    output.push(AstNode::Function {
                                        name,
                                        args: Vec::new(),
                                    });
                                }
                            }
                            break;
                        }
                        Some(other) => emit(other, &mut output),
                        None => {
                            return Err(CalcError::new(
                                ErrorKind::ParseError,
                                "Paréntesis derecho sin correspondencia".to_string(),
                            ));
                        }
                    }
                }
            }

            // ── Factorial (postfijo) ────────────────────────────────────
            Token::Factorial => {
                output.push(AstNode::UnaryOp {
                    op: UnaryOperator::Factorial,
                    operand: dummy_number(),
                });
            }

            // ── Negación bitwise (siempre prefijo unario) ────────────
            Token::BitNot => {
                op_stack.push(StackItem::UnaryOp(UnaryOperator::BitNot));
            }

            // ── Operadores ──────────────────────────────────────────────
            Token::Plus | Token::Minus | Token::Star | Token::Slash
            | Token::Caret | Token::Percent
            | Token::BitAnd | Token::BitOr | Token::Xor
            | Token::Shl | Token::Shr => {
                let is_unary_minus =
                    *token == Token::Minus && is_unary_context(idx, tokens);

                if is_unary_minus {
                    op_stack.push(StackItem::UnaryOp(UnaryOperator::Negate));
                } else {
                    let bin_op = token_to_binary_op(token);

                    // Desapilar operadores con mayor o igual precedencia.
                    while let Some(top) = op_stack.last() {
                        if should_pop_stack(top, &bin_op) {
                            let item = op_stack.pop().unwrap();
                            emit(item, &mut output);
                        } else {
                            break;
                        }
                    }
                    op_stack.push(StackItem::BinOp(bin_op));
                }
            }
        }

        idx += 1;
    }

    // Verificar balance final de paréntesis.
    if paren_balance != 0 {
        return Err(CalcError::new(
            ErrorKind::ParseError,
            "Paréntesis sin cerrar".to_string(),
        ));
    }

    // Vaciar la pila de operadores restante.
    while let Some(item) = op_stack.pop() {
        match item {
            StackItem::LParen => {
                return Err(CalcError::new(
                    ErrorKind::ParseError,
                    "Paréntesis sin cerrar".to_string(),
                ));
            }
            other => emit(other, &mut output),
        }
    }

    Ok(output)
}

/// Convierte un token de operador en su representación `BinaryOperator`.
fn token_to_binary_op(token: &Token) -> BinaryOperator {
    match token {
        Token::Plus => BinaryOperator::Add,
        Token::Minus => BinaryOperator::Sub,
        Token::Star => BinaryOperator::Mul,
        Token::Slash => BinaryOperator::Div,
        Token::Caret => BinaryOperator::Pow,
        Token::Percent => BinaryOperator::Mod,
        Token::BitAnd => BinaryOperator::BitAnd,
        Token::BitOr => BinaryOperator::BitOr,
        Token::Xor => BinaryOperator::BitXor,
        Token::Shl => BinaryOperator::Shl,
        Token::Shr => BinaryOperator::Shr,
        _ => unreachable!("token_to_binary_op llamado con {:?}", token),
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tokenizer::tokenize;

    fn parse(expr: &str) -> Result<Vec<AstNode>, CalcError> {
        let tokens = tokenize(expr)?;
        shunting_yard(&tokens)
    }

    fn count_nodes<F>(nodes: &[AstNode], predicate: F) -> usize
    where
        F: Fn(&AstNode) -> bool,
    {
        nodes.iter().filter(|n| predicate(n)).count()
    }

    #[test]
    fn test_simple_addition_rpn() {
        // 2+3 → RPN: [2, 3, +]
        let nodes = parse("2+3").unwrap();
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[0], AstNode::Number(2.0)));
        assert!(matches!(&nodes[1], AstNode::Number(3.0)));
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
    }

    #[test]
    fn test_subtraction_rpn() {
        let nodes = parse("5-3").unwrap();
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Sub, .. }));
    }

    #[test]
    fn test_multiplication_rpn() {
        let nodes = parse("4*5").unwrap();
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Mul, .. }));
    }

    #[test]
    fn test_division_rpn() {
        let nodes = parse("8/2").unwrap();
        assert_eq!(nodes.len(), 3);
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Div, .. }));
    }

    #[test]
    fn test_precedence_multiplication_over_addition() {
        // 2+3*4 = 14 → RPN: [2, 3, 4, *, +]
        let nodes = parse("2+3*4").unwrap();
        assert_eq!(nodes.len(), 5);
        // Operador * antes que + en RPN
        assert!(matches!(&nodes[3], AstNode::BinaryOp { op: BinaryOperator::Mul, .. }));
        assert!(matches!(&nodes[4], AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
    }

    #[test]
    fn test_parentheses_override_precedence() {
        // (2+3)*4 = 20 → RPN: [2, 3, +, 4, *]
        let nodes = parse("(2+3)*4").unwrap();
        assert_eq!(nodes.len(), 5);
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
        assert!(matches!(&nodes[4], AstNode::BinaryOp { op: BinaryOperator::Mul, .. }));
    }

    #[test]
    fn test_nested_parentheses() {
        // ((2+3)*(4+5)) → RPN: [2, 3, +, 4, 5, +, *]
        let nodes = parse("((2+3)*(4+5))").unwrap();
        assert_eq!(nodes.len(), 7);
    }

    #[test]
    fn test_right_associative_power() {
        // 2^3^2 = 2^(3^2) = 512
        // RPN con asociatividad derecha: [2, 3, 2, ^, ^]
        let nodes = parse("2^3^2").unwrap();
        assert_eq!(nodes.len(), 5);
        assert!(matches!(&nodes[0], AstNode::Number(2.0)));
        assert!(matches!(&nodes[1], AstNode::Number(3.0)));
        assert!(matches!(&nodes[2], AstNode::Number(2.0)));
        assert!(matches!(&nodes[3], AstNode::BinaryOp { op: BinaryOperator::Pow, .. }));
        assert!(matches!(&nodes[4], AstNode::BinaryOp { op: BinaryOperator::Pow, .. }));
    }

    #[test]
    fn test_unary_negate_of_number() {
        // -5+3: -5 se tokeniza como Number(-5), así que no hay Negate en RPN.
        let nodes = parse("-5+3").unwrap();
        assert_eq!(count_nodes(&nodes, |n| matches!(n, AstNode::Number(_))), 2);
        assert_eq!(
            count_nodes(&nodes, |n| matches!(n, AstNode::UnaryOp { op: UnaryOperator::Negate, .. })),
            0
        );
    }

    #[test]
    fn test_unary_negate_of_expression() {
        // -(2+3) → RPN: [2, 3, +, Negate]
        let nodes = parse("-(2+3)").unwrap();
        assert_eq!(nodes.len(), 4);
        assert!(matches!(&nodes[3], AstNode::UnaryOp { op: UnaryOperator::Negate, .. }));
    }

    #[test]
    fn test_factorial_rpn() {
        let nodes = parse("5!").unwrap();
        assert_eq!(nodes.len(), 2);
        assert!(matches!(&nodes[0], AstNode::Number(5.0)));
        assert!(matches!(&nodes[1], AstNode::UnaryOp { op: UnaryOperator::Factorial, .. }));
    }

    #[test]
    fn test_factorial_with_expression() {
        // (2+3)! → RPN: [2, 3, +, !]
        let nodes = parse("(2+3)!").unwrap();
        assert!(matches!(
            nodes.last().unwrap(),
            &AstNode::UnaryOp { op: UnaryOperator::Factorial, .. }
        ));
    }

    #[test]
    fn test_simple_function() {
        let nodes = parse("sin(0)").unwrap();
        let has_sin = nodes.iter().any(|n| {
            matches!(n, AstNode::Function { name, .. } if name == "sin")
        });
        assert!(has_sin);
    }

    #[test]
    fn test_function_with_expression() {
        // sin(pi/2) → RPN: [pi, 2, /, sin]
        let nodes = parse("sin(pi/2)").unwrap();
        assert_eq!(nodes.len(), 4);
        assert!(matches!(&nodes[0], AstNode::Constant(c) if c == "pi"));
        assert!(matches!(&nodes[1], AstNode::Number(2.0)));
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Div, .. }));
        assert!(matches!(&nodes[3], AstNode::Function { name, .. } if name == "sin"));
    }

    #[test]
    fn test_constants() {
        let nodes = parse("pi + e").unwrap();
        assert!(matches!(&nodes[0], AstNode::Constant(c) if c == "pi"));
        assert!(matches!(&nodes[1], AstNode::Constant(c) if c == "e"));
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
    }

    #[test]
    fn test_ans_constant() {
        let nodes = parse("ans * 2").unwrap();
        assert!(matches!(&nodes[0], AstNode::Constant(c) if c == "ans"));
    }

    #[test]
    fn test_module_operator() {
        let nodes = parse("10 % 3").unwrap();
        assert!(matches!(&nodes[2], AstNode::BinaryOp { op: BinaryOperator::Mod, .. }));
    }

    #[test]
    fn test_unbalanced_left_parens_error() {
        let result = parse("(2+3");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
    }

    #[test]
    fn test_unbalanced_right_parens_error() {
        let result = parse("2+3)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
    }

    #[test]
    fn test_empty_input() {
        let nodes = parse("").unwrap();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_complex_expression() {
        // sin(pi/2) + 3! * 2^3
        let nodes = parse("sin(pi/2)+3!*2^3").unwrap();
        assert!(!nodes.is_empty());
        // Debe contener sin, factorial, potencia, suma, multiplicación
        assert!(nodes.iter().any(|n| matches!(n, AstNode::Function { name, .. } if name == "sin")));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, AstNode::UnaryOp { op: UnaryOperator::Factorial, .. })));
        assert!(nodes
            .iter()
            .any(|n| matches!(n, AstNode::BinaryOp { op: BinaryOperator::Pow, .. })));
    }

    #[test]
    fn test_negate_of_function() {
        // -sin(0)
        let nodes = parse("-sin(0)").unwrap();
        // Debe haber Negate al final (RPN: 0, sin, Negate)
        assert!(matches!(
            nodes.last().unwrap(),
            &AstNode::UnaryOp { op: UnaryOperator::Negate, .. }
        ));
    }
}
