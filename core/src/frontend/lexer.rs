use crate::notify::Message;
use crate::types::tokens;
use logos::{Lexer, Logos};
use std::process;

// Lexer the input and returns a vector of tokens
pub fn lex_source(input: &str) -> Vec<tokens::Token> {
    let mut lex = tokens::TokenType::lexer(input);
    let mut token_position = tokens::Position { line: 0, column: 0 };
    let mut result = vec![];

    while let Some(token_type) = lex.next() {
        match token_type {
            Ok(token) => {
                // If the token type is a new line token, so update the token position
                match token {
                    tokens::TokenType::NewLine => token_position.line += 1,
                    _ => token_position.column = lex.span().start - lex.extras.1,
                }

                // A new token
                let mut tk = tokens::Token {
                    token_type: token.clone(),
                    token_value: lex.slice().to_string(),
                    line: token_position.line,
                    column: token_position.column,
                };

                // Insert the new token in the result vector
                result.insert(result.len(), tk);
            }
            Err(_) => {
                eprintln!(
                    "- [error] -> ({}:{}): {} undefined token.",
                    token_position.line,
                    lex.span().start - lex.extras.1,
                    lex.slice()
                )
            }
        }
    }

    return lex_trim_result(result);
}

pub fn lex_trim_result(input: Vec<tokens::Token>) -> Vec<tokens::Token> {
    let mut result = vec![];

    for token in input {
        match token.token_type {
            tokens::TokenType::NewLine => continue,
            tokens::TokenType::Space => continue,
            _ => result.insert(result.len(), token),
        }
    }

    return result;
}
