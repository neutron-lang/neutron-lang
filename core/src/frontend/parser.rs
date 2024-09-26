use crate::types::{bult_in_types::*, others::*, parse_nodes::*, tokens::*};
use std::{iter::Peekable, process::exit};

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current_token: Token,
    index: usize,
    pub ast: Statement,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        let mut input_iter = input.into_iter().peekable();
        Self {
            current_token: input_iter.next().unwrap(),
            index: 0,
            tokens: input_iter,
            ast: Statement::Program {
                start: Position { line: 1, column: 0 },
                body: Box::new(Vec::new()),
            },
        }
    }

    pub fn parse_tokens(&mut self) {
        while self.index != self.tokens.len() {
            self.index += 1;

            let ast_node = match &self.current_type() {
                TokenType::NewLine => todo!(),
                TokenType::Space => todo!(),
                TokenType::Dot => todo!(),
                TokenType::Comma => todo!(),
                TokenType::Colon => todo!(),
                TokenType::SemiColon => todo!(),
                TokenType::SingleQuote => todo!(),
                TokenType::DoubleQuotes => todo!(),
                TokenType::LParen => todo!(),
                TokenType::RParen => todo!(),
                TokenType::LBrace => todo!(),
                TokenType::RBrace => todo!(),
                TokenType::LBracket => todo!(),
                TokenType::RBracket => todo!(),
                TokenType::OpenComment => todo!(),
                TokenType::CloseComment => todo!(),
                TokenType::KwNamespace => todo!(),
                TokenType::KwImport => todo!(),
                TokenType::KwVar => todo!(),
                TokenType::KwConst => todo!(),
                TokenType::KwClass => todo!(),
                TokenType::KwStruct => todo!(),
                TokenType::KwFunc => todo!(),
                TokenType::KwIf => todo!(),
                TokenType::KwElse => todo!(),
                TokenType::KwElif => todo!(),
                TokenType::KwLoop => todo!(),
                TokenType::KwWhile => todo!(),
                TokenType::KwFor => todo!(),
                TokenType::KwContinue => todo!(),
                TokenType::KwBreak => todo!(),
                TokenType::KwReturn => todo!(),
                TokenType::Null => todo!(),
                TokenType::Int => todo!(),
                TokenType::Float => todo!(),
                TokenType::Bool => todo!(),
                TokenType::Str => todo!(),
                TokenType::Void => todo!(),
                TokenType::Any => todo!(),
                TokenType::OpPlus => todo!(),
                TokenType::OpMinus => todo!(),
                TokenType::OpAsterisk => todo!(),
                TokenType::OpDivision => todo!(),
                TokenType::OpRest => todo!(),
                TokenType::OpNot => todo!(),
                TokenType::OpAnd => todo!(),
                TokenType::OpOr => todo!(),
                TokenType::OpSmallerThan => todo!(),
                TokenType::OpGreaterThan => todo!(),
                TokenType::OpSmallerOrEqualsThan => todo!(),
                TokenType::OpGreaterOrEqualsThan => todo!(),
                TokenType::OpEquals => todo!(),
                TokenType::OpNotEquals => todo!(),
                TokenType::OpAssign => todo!(),
                TokenType::OpAssignPlus => todo!(),
                TokenType::OpAssignMinus => todo!(),
                TokenType::OpAssignMultiply => todo!(),
                TokenType::OpAssignDivision => todo!(),
                TokenType::OpAssignRest => todo!(),
                TokenType::OpSetOrAcess => todo!(),
                TokenType::Identifier => todo!(),
                TokenType::Number => todo!(),
            };
        }
    }

    fn current(&self) -> &Token {
        &self.current_token
    }

    fn current_type(&mut self) -> &TokenType {
        &self.current().token_type
    }
}
