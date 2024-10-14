use crate::notify::*;
use crate::types::others::*;
use crate::types::tokens::*;
use logos::{Lexer, Logos};
use std::process;

/// Update the line count and the char index.
pub fn newline_callback(lex: &mut Lexer<TokenType>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

/// Compute the line and column position for the current word.
pub fn word_callback(lex: &mut Lexer<TokenType>) {
    let _line = lex.extras.0;
    let _column = lex.span().start - lex.extras.1;
}

/// Lexer the input and returns a vector of tokens
pub fn lex_source(input: &str) -> Vec<Token> {
    let mut lex = TokenType::lexer(input);
    let mut token_position = Position { line: 0, column: 0 };
    let mut result: Vec<Token> = vec![];

    let mut message = Message {
        text: String::new(),
        line: 0,
        column: 0,
    };
    let mut error_count: usize = 0;
    let mut can_proceed = true;

    while let Some(token_type) = lex.next() {
        match token_type {
            Ok(token) => {
                // If the token type is a new line token, so update the token position
                match token {
                    TokenType::NewLine => token_position.line += 1,
                    _ => token_position.column = lex.span().start - lex.extras.1,
                }

                // A new token
                let tk = Token {
                    token_type: token.clone(),
                    token_value: lex.slice().to_string(),
                    line: token_position.line,
                    column: token_position.column,
                };

                // Insert the new token in the result vector
                result.insert(result.len(), tk);
            }
            Err(_) => {
                message.text = String::from(format!("{:?} -> non existent token.", lex.slice()));
                message.line = lex.extras.0;
                message.column = lex.span().start - lex.extras.1;
                message.show_error();

                error_count += 1;
                can_proceed = false;
            }
        }
    }

    result.insert(
        result.len(),
        Token {
            token_type: TokenType::Eof,
            token_value: String::from("EOF"),
            line: token_position.line.clone(),
            column: token_position.column.clone() + 1,
        },
    );

    if can_proceed {
        result = lex_trim_result(result);
    } else {
        message.text = String::from(format!("Can't proceed due by {} errors.", error_count));
        message.show_message("compiler".to_string());
        process::exit(1);
    }

    return result;
}

/// Remove unused spaces and new lines
pub fn lex_trim_result(input: Vec<Token>) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    for token in input {
        match token.token_type {
            TokenType::NewLine => continue,
            TokenType::Space => continue,
            _ => result.insert(result.len(), token),
        }
    }

    return result;
}
