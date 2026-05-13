//! Tokenizador de expresiones matemáticas.
//! Convierte una cadena de entrada en un vector de tokens usando
//! un escáner carácter por carácter.

use crate::models::errors::{CalcError, ErrorKind};

/// Tokens que componen una expresión matemática.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Percent,
    LParen,
    RParen,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Log,
    Ln,
    Log2,
    Log10,
    Sqrt,
    Cbrt,
    Abs,
    Exp,
    Factorial,
    Pi,
    E,
    Ans,
    Comma,
    /// Operadores bitwise.
    BitAnd,
    BitOr,
    BitNot,
    Shl,
    Shr,
    Xor,
    /// Unidad imaginaria `i` o `j`.
    Imaginary,
    /// Funciones complejas.
    Real,
    Imag,
    Conj,
    Arg,
}

/// Convierte una cadena de expresión en un vector de tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut pos = 0;
    let mut tokens = Vec::new();

    // Determina si un token `-` debe interpretarse como unario (negación)
    // basándose en el token anterior.
    fn is_unary_minus_context(last_token: Option<&Token>) -> bool {
        match last_token {
            None => true, // inicio de expresión
            Some(Token::Plus)
            | Some(Token::Minus)
            | Some(Token::Star)
            | Some(Token::Slash)
            | Some(Token::Caret)
            | Some(Token::Percent)
            | Some(Token::LParen)
            | Some(Token::Comma)
            | Some(Token::BitAnd)
            | Some(Token::BitOr)
            | Some(Token::BitNot)
            | Some(Token::Shl)
            | Some(Token::Shr)
            | Some(Token::Xor) => true,
            _ => false,
        }
    }

    while pos < len {
        let ch = chars[pos];

        // Saltar espacios en blanco
        if ch.is_whitespace() {
            pos += 1;
            continue;
        }

        // Prefijos de base: 0x (hex), 0o (oct), 0b (bin)
        if ch == '0' && pos + 1 < len {
            let next = chars[pos + 1];
            if next == 'x' || next == 'X' {
                pos += 2;
                let start = pos;
                while pos < len && chars[pos].is_ascii_hexdigit() {
                    pos += 1;
                }
                if pos == start {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        format!("Número hexadecimal sin dígitos en posición {}", start - 2),
                    )
                    .with_position(start - 2));
                }
                let hex_str: String = chars[start..pos].iter().collect();
                let num = i64::from_str_radix(&hex_str, 16).map_err(|_| {
                    CalcError::new(
                        ErrorKind::ParseError,
                        format!("Número hexadecimal inválido '0x{}' en posición {}", hex_str, start - 2),
                    )
                    .with_position(start - 2)
                })? as f64;
                tokens.push(Token::Number(num));
                continue;
            }
            if next == 'o' || next == 'O' {
                pos += 2;
                let start = pos;
                while pos < len && chars[pos] >= '0' && chars[pos] <= '7' {
                    pos += 1;
                }
                if pos == start {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        format!("Número octal sin dígitos en posición {}", start - 2),
                    )
                    .with_position(start - 2));
                }
                let oct_str: String = chars[start..pos].iter().collect();
                let num = i64::from_str_radix(&oct_str, 8).map_err(|_| {
                    CalcError::new(
                        ErrorKind::ParseError,
                        format!("Número octal inválido '0o{}' en posición {}", oct_str, start - 2),
                    )
                    .with_position(start - 2)
                })? as f64;
                tokens.push(Token::Number(num));
                continue;
            }
            if next == 'b' || next == 'B' {
                pos += 2;
                let start = pos;
                while pos < len && (chars[pos] == '0' || chars[pos] == '1') {
                    pos += 1;
                }
                if pos == start {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        format!("Número binario sin dígitos en posición {}", start - 2),
                    )
                    .with_position(start - 2));
                }
                let bin_str: String = chars[start..pos].iter().collect();
                let num = i64::from_str_radix(&bin_str, 2).map_err(|_| {
                    CalcError::new(
                        ErrorKind::ParseError,
                        format!("Número binario inválido '0b{}' en posición {}", bin_str, start - 2),
                    )
                    .with_position(start - 2)
                })? as f64;
                tokens.push(Token::Number(num));
                continue;
            }
        }

        // Números (incluyendo notación científica y negativos unarios)
        if ch.is_ascii_digit() || ch == '.' {
            let start = pos;
            let mut has_dot = ch == '.';
            pos += 1;

            // Consumir dígitos y punto decimal
            while pos < len && (chars[pos].is_ascii_digit() || chars[pos] == '.') {
                if chars[pos] == '.' {
                    if has_dot {
                        return Err(CalcError::new(
                            ErrorKind::ParseError,
                            format!("Segundo punto decimal en número en posición {}", pos),
                        )
                        .with_position(pos));
                    }
                    has_dot = true;
                }
                pos += 1;
            }

            // Notación científica: 1e10, 1.5e-3, 2E+4
            if pos < len && (chars[pos] == 'e' || chars[pos] == 'E') {
                pos += 1;
                // Signo opcional del exponente
                if pos < len && (chars[pos] == '+' || chars[pos] == '-') {
                    pos += 1;
                }
                // Debe haber al menos un dígito en el exponente
                if pos >= len || !chars[pos].is_ascii_digit() {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        format!("Notación científica incompleta en posición {}", start),
                    )
                    .with_position(start));
                }
                while pos < len && chars[pos].is_ascii_digit() {
                    pos += 1;
                }
            }

            let num_str: String = chars[start..pos].iter().collect();
            let num = num_str.parse::<f64>().map_err(|_| {
                CalcError::new(
                    ErrorKind::ParseError,
                    format!("Número inválido '{}' en posición {}", num_str, start),
                )
                .with_position(start)
            })?;

            tokens.push(Token::Number(num));
            continue;
        }

        // Guión: puede ser resta o negación unaria
        if ch == '-' {
            if is_unary_minus_context(tokens.last()) {
                // Negación unaria: ver si le sigue un número
                let next_pos = pos + 1;
                if next_pos < len && (chars[next_pos].is_ascii_digit() || chars[next_pos] == '.') {
                    // Número negativo: parsearlo como un solo token Number
                    pos += 1; // saltar el signo
                    let start = pos;
                    let mut has_dot = chars[pos] == '.';
                    pos += 1;
                    while pos < len && (chars[pos].is_ascii_digit() || chars[pos] == '.') {
                        if chars[pos] == '.' {
                            if has_dot {
                                return Err(CalcError::new(
                                    ErrorKind::ParseError,
                                    format!(
                                        "Segundo punto decimal en número en posición {}",
                                        pos
                                    ),
                                )
                                .with_position(pos));
                            }
                            has_dot = true;
                        }
                        pos += 1;
                    }
                    // Notación científica
                    if pos < len && (chars[pos] == 'e' || chars[pos] == 'E') {
                        pos += 1;
                        if pos < len && (chars[pos] == '+' || chars[pos] == '-') {
                            pos += 1;
                        }
                        if pos >= len || !chars[pos].is_ascii_digit() {
                            return Err(CalcError::new(
                                ErrorKind::ParseError,
                                format!("Notación científica incompleta en posición {}", start - 1),
                            )
                            .with_position(start - 1));
                        }
                        while pos < len && chars[pos].is_ascii_digit() {
                            pos += 1;
                        }
                    }
                    let num_str: String = chars[(start - 1)..pos].iter().collect();
                    let num = num_str.parse::<f64>().map_err(|_| {
                        CalcError::new(
                            ErrorKind::ParseError,
                            format!("Número inválido '{}' en posición {}", num_str, start - 1),
                        )
                        .with_position(start - 1)
                    })?;
                    tokens.push(Token::Number(num));
                    continue;
                }
                // Negación unaria de expresión (ej: -(2+3), -sin(x))
                // Emitimos un token especial que el shunting yard procesará.
                // Usamos un flag en el shunting yard, aquí simplemente emitimos
                // Minus y dejamos que el parser de contexto decida.
                // Para simplificar, insertamos un marcador de negación unaria.
                tokens.push(Token::Minus);
                pos += 1;
                continue;
            } else {
                tokens.push(Token::Minus);
                pos += 1;
                continue;
            }
        }

        // Operadores y símbolos simples
        match ch {
            '+' => {
                tokens.push(Token::Plus);
                pos += 1;
                continue;
            }
            '*' => {
                tokens.push(Token::Star);
                pos += 1;
                continue;
            }
            '/' => {
                tokens.push(Token::Slash);
                pos += 1;
                continue;
            }
            '^' => {
                tokens.push(Token::Caret);
                pos += 1;
                continue;
            }
            '%' => {
                tokens.push(Token::Percent);
                pos += 1;
                continue;
            }
            '(' => {
                tokens.push(Token::LParen);
                pos += 1;
                continue;
            }
            ')' => {
                tokens.push(Token::RParen);
                pos += 1;
                continue;
            }
            '!' => {
                tokens.push(Token::Factorial);
                pos += 1;
                continue;
            }
            ',' => {
                tokens.push(Token::Comma);
                pos += 1;
                continue;
            }
            '&' => {
                tokens.push(Token::BitAnd);
                pos += 1;
                continue;
            }
            '|' => {
                tokens.push(Token::BitOr);
                pos += 1;
                continue;
            }
            '~' => {
                tokens.push(Token::BitNot);
                pos += 1;
                continue;
            }
            '<' => {
                if pos + 1 < len && chars[pos + 1] == '<' {
                    tokens.push(Token::Shl);
                    pos += 2;
                } else {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        format!("Operador '<' no soportado en posición {}", pos),
                    ).with_position(pos));
                }
                continue;
            }
            '>' => {
                if pos + 1 < len && chars[pos + 1] == '>' {
                    tokens.push(Token::Shr);
                    pos += 2;
                } else {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        format!("Operador '>' no soportado en posición {}", pos),
                    ).with_position(pos));
                }
                continue;
            }
            _ => {}
        }

        // Identificadores: funciones y constantes (letras)
        if ch.is_alphabetic() || ch == '_' {
            let start = pos;
            pos += 1;
            while pos < len && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                pos += 1;
            }
            let ident: String = chars[start..pos].iter().collect();
            let lower = ident.to_lowercase();

            match lower.as_str() {
                "sin" => tokens.push(Token::Sin),
                "cos" => tokens.push(Token::Cos),
                "tan" => tokens.push(Token::Tan),
                "asin" | "arcsin" => tokens.push(Token::Asin),
                "acos" | "arccos" => tokens.push(Token::Acos),
                "atan" | "arctan" => tokens.push(Token::Atan),
                "sinh" => tokens.push(Token::Sinh),
                "cosh" => tokens.push(Token::Cosh),
                "tanh" => tokens.push(Token::Tanh),
                "log" => tokens.push(Token::Log),
                "ln" => tokens.push(Token::Ln),
                "log2" => tokens.push(Token::Log2),
                "log10" => tokens.push(Token::Log10),
                "sqrt" => tokens.push(Token::Sqrt),
                "cbrt" => tokens.push(Token::Cbrt),
                "abs" => tokens.push(Token::Abs),
                "exp" => tokens.push(Token::Exp),
                "pi" => tokens.push(Token::Pi),
                "e" => tokens.push(Token::E),
                "ans" => tokens.push(Token::Ans),
                "xor" => tokens.push(Token::Xor),
                "i" | "j" => tokens.push(Token::Imaginary),
                "real" => tokens.push(Token::Real),
                "imag" => tokens.push(Token::Imag),
                "conj" => tokens.push(Token::Conj),
                "arg" => tokens.push(Token::Arg),
                _ => {
                    return Err(CalcError::new(
                        ErrorKind::ParseError,
                        format!("Token no reconocido '{}' en posición {}", ident, start),
                    )
                    .with_position(start));
                }
            }
            continue;
        }

        // Carácter no reconocido
        return Err(CalcError::new(
            ErrorKind::ParseError,
            format!("Carácter inesperado '{}' en posición {}", ch, pos),
        )
        .with_position(pos));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_number() {
        let tokens = tokenize("42").unwrap();
        assert_eq!(tokens, vec![Token::Number(42.0)]);
    }

    #[test]
    fn test_decimal_number() {
        let tokens = tokenize("3.14159").unwrap();
        assert_eq!(tokens, vec![Token::Number(3.14159)]);
    }

    #[test]
    fn test_negative_number() {
        let tokens = tokenize("-5").unwrap();
        assert_eq!(tokens, vec![Token::Number(-5.0)]);
    }

    #[test]
    fn test_scientific_notation() {
        let tokens = tokenize("1e10").unwrap();
        assert_eq!(tokens, vec![Token::Number(1e10)]);

        let tokens = tokenize("1.5e-3").unwrap();
        assert_eq!(tokens, vec![Token::Number(1.5e-3)]);

        let tokens = tokenize("2E+4").unwrap();
        assert_eq!(tokens, vec![Token::Number(2e4)]);
    }

    #[test]
    fn test_basic_operators() {
        let tokens = tokenize("2 + 3 * 4").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.0),
                Token::Star,
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn test_parentheses() {
        let tokens = tokenize("(2+3)*4").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.0),
                Token::RParen,
                Token::Star,
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn test_functions() {
        let tokens = tokenize("sin(0) + cos(pi)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Sin,
                Token::LParen,
                Token::Number(0.0),
                Token::RParen,
                Token::Plus,
                Token::Cos,
                Token::LParen,
                Token::Pi,
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_constants() {
        let tokens = tokenize("pi + e + ans").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Pi, Token::Plus, Token::E, Token::Plus, Token::Ans,]
        );
    }

    #[test]
    fn test_factorial() {
        let tokens = tokenize("5!").unwrap();
        assert_eq!(tokens, vec![Token::Number(5.0), Token::Factorial,]);
    }

    #[test]
    fn test_power() {
        let tokens = tokenize("2^3^2").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(2.0),
                Token::Caret,
                Token::Number(3.0),
                Token::Caret,
                Token::Number(2.0),
            ]
        );
    }

    #[test]
    fn test_negative_expression() {
        // - (2+3) debe tokenizar como: Minus, LParen, Num, Plus, Num, RParen
        let tokens = tokenize("-(2+3)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Minus,
                Token::LParen,
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_all_functions() {
        let funcs = [
            ("sin", Token::Sin),
            ("cos", Token::Cos),
            ("tan", Token::Tan),
            ("asin", Token::Asin),
            ("acos", Token::Acos),
            ("atan", Token::Atan),
            ("sinh", Token::Sinh),
            ("cosh", Token::Cosh),
            ("tanh", Token::Tanh),
            ("log", Token::Log),
            ("ln", Token::Ln),
            ("log2", Token::Log2),
            ("log10", Token::Log10),
            ("sqrt", Token::Sqrt),
            ("cbrt", Token::Cbrt),
            ("abs", Token::Abs),
            ("exp", Token::Exp),
        ];
        for (name, expected) in &funcs {
            let tokens = tokenize(&format!("{}(1)", name)).unwrap();
            assert_eq!(tokens[0], *expected, "Failed for function: {}", name);
        }
    }

    #[test]
    fn test_unknown_token_error() {
        let result = tokenize("2 + @");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::ParseError);
        assert!(err.position.is_some());
    }

    #[test]
    fn test_empty_input() {
        let tokens = tokenize("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_whitespace_handling() {
        let tokens = tokenize("  2   +   3  ").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(2.0), Token::Plus, Token::Number(3.0),]
        );
    }

    #[test]
    fn test_unary_minus_with_function() {
        // -sin(0) debe interpretarse como negación unaria de sin
        let tokens = tokenize("-sin(0)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Minus,
                Token::Sin,
                Token::LParen,
                Token::Number(0.0),
                Token::RParen,
            ]
        );
    }

    // ─── Prefijos de base ─────────────────────────────────────────────

    #[test]
    fn test_hex_prefix() {
        let tokens = tokenize("0xFF").unwrap();
        assert_eq!(tokens, vec![Token::Number(255.0)]);
    }

    #[test]
    fn test_hex_lowercase() {
        let tokens = tokenize("0xff").unwrap();
        assert_eq!(tokens, vec![Token::Number(255.0)]);
    }

    #[test]
    fn test_hex_mixed_case() {
        let tokens = tokenize("0x1A2b").unwrap();
        assert_eq!(tokens, vec![Token::Number(6699.0)]);
    }

    #[test]
    fn test_oct_prefix() {
        let tokens = tokenize("0o77").unwrap();
        assert_eq!(tokens, vec![Token::Number(63.0)]);
    }

    #[test]
    fn test_oct_uppercase() {
        let tokens = tokenize("0O10").unwrap();
        assert_eq!(tokens, vec![Token::Number(8.0)]);
    }

    #[test]
    fn test_bin_prefix() {
        let tokens = tokenize("0b1010").unwrap();
        assert_eq!(tokens, vec![Token::Number(10.0)]);
    }

    #[test]
    fn test_bin_uppercase() {
        let tokens = tokenize("0B0101").unwrap();
        assert_eq!(tokens, vec![Token::Number(5.0)]);
    }

    #[test]
    fn test_bin_large() {
        let tokens = tokenize("0b11111111111111111111111111111111").unwrap();
        assert_eq!(tokens, vec![Token::Number(4294967295.0)]);
    }

    #[test]
    fn test_hex_no_digits_error() {
        let result = tokenize("0x");
        assert!(result.is_err());
    }

    #[test]
    fn test_oct_no_digits_error() {
        let result = tokenize("0o");
        assert!(result.is_err());
    }

    #[test]
    fn test_bin_no_digits_error() {
        let result = tokenize("0b");
        assert!(result.is_err());
    }

    // ─── Operadores bitwise ─────────────────────────────────────────────

    #[test]
    fn test_bitwise_and() {
        let tokens = tokenize("5 & 3").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(5.0),
                Token::BitAnd,
                Token::Number(3.0),
            ]
        );
    }

    #[test]
    fn test_bitwise_or() {
        let tokens = tokenize("5 | 3").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(5.0),
                Token::BitOr,
                Token::Number(3.0),
            ]
        );
    }

    #[test]
    fn test_bitwise_not() {
        let tokens = tokenize("~5").unwrap();
        assert_eq!(tokens, vec![Token::BitNot, Token::Number(5.0)]);
    }

    #[test]
    fn test_shift_left() {
        let tokens = tokenize("1 << 4").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.0),
                Token::Shl,
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn test_shift_right() {
        let tokens = tokenize("16 >> 2").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(16.0),
                Token::Shr,
                Token::Number(2.0),
            ]
        );
    }

    #[test]
    fn test_xor_keyword() {
        let tokens = tokenize("5 xor 3").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(5.0),
                Token::Xor,
                Token::Number(3.0),
            ]
        );
    }

    // ─── Unidad imaginaria y funciones complejas ────────────────────────

    #[test]
    fn test_imaginary_unit_i() {
        let tokens = tokenize("i").unwrap();
        assert_eq!(tokens, vec![Token::Imaginary]);
    }

    #[test]
    fn test_imaginary_unit_j() {
        let tokens = tokenize("j").unwrap();
        assert_eq!(tokens, vec![Token::Imaginary]);
    }

    #[test]
    fn test_real_function() {
        let tokens = tokenize("real(5)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Real,
                Token::LParen,
                Token::Number(5.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_conj_function() {
        let tokens = tokenize("conj(3+4i)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Conj,
                Token::LParen,
                Token::Number(3.0),
                Token::Plus,
                Token::Number(4.0),
                Token::Imaginary,
                Token::RParen,
            ]
        );
    }
}
