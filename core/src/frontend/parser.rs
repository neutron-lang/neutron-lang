use crate::types::tokens::*;

#[derive(Debug, Clone)]
pub struct Parser {
    lexer_result: Vec<Token>,
    current_position: usize,
}

impl Parser {
    pub fn new(lexer_result: Vec<Token>) -> Parser {
        let new_parser = Parser {
            lexer_result,
            current_position: 0,
        };

        return new_parser;
    }
}
