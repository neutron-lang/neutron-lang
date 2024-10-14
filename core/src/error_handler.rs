use crate::notify::Message;
use crate::types::tokens::*;
use std::process::exit;

/// [E01]
pub fn expected_error(expected: &str, found: &Token) -> ! {
    Message {
        text: String::from(format!(
            "expected {} but found {}",
            expected, found.token_value
        )),
        line: found.line,
        column: found.column,
    }
    .show_error();

    exit(1)
}

/// [E02]
pub fn expected_or_error(expected_type: &TokenType, expected: &str, found: &Token) {
    if !found.token_type.eq(expected_type) {
        Message {
            text: String::from(format!(
                "expected {} but found {}",
                expected, found.token_value
            )),
            line: found.line,
            column: found.column,
        }
        .show_error();

        exit(1);
    }
}

/// [E03]
pub fn expect_expression_or_error(token: &Token) {
    match token.token_type {
        TokenType::Identifier
        | TokenType::Number
        | TokenType::OpPlus
        | TokenType::OpMinus
        | TokenType::OpNot
        | TokenType::CharLiteral
        | TokenType::StringLiteral
        | TokenType::True
        | TokenType::False
        | TokenType::Null
        | TokenType::LBrace
        | TokenType::LParen => {}
        TokenType::Eof => expected_error("expression", &token.to_owned()),
        _ => expected_error("expression", &token.to_owned()),
    }
}
