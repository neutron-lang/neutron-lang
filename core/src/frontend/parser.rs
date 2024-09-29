use crate::{
    notify,
    types::{bult_in_types::*, others::*, parse_nodes::*, tokens::*},
};
use std::{iter::Peekable, process::exit, ptr::eq};

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current_token: Token,
    pub ast: Statement,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        let mut input_iter = input.into_iter().peekable();
        Self {
            current_token: input_iter.next().unwrap(),
            tokens: input_iter,
            ast: Statement::Program {
                start: Position { line: 1, column: 0 },
                body: Box::new(Vec::new()),
            },
        }
    }

    pub fn parse_tokens(&mut self) {
        while !self.current_type().eq(&TokenType::Eof) {
            let ast_node = match &self.current_type() {
                TokenType::KwFunc => self.parse_function_statement(),
                _ => {}
            };

            self.advance();
        }
    }

    fn current(&self) -> &Token {
        &self.current_token
    }

    fn current_type(&mut self) -> &TokenType {
        &self.current().token_type
    }

    fn advance(&mut self) {
        match self.current().token_type {
            TokenType::Eof => {}
            _ => self.current_token = self.tokens.next().unwrap(),
        }
    }

    /// Return the current token type or the eof
    fn peek_type(&mut self) -> &TokenType {
        match self.tokens.peek() {
            Some(token) => &token.token_type,
            None => &TokenType::Eof,
        }
    }

    /// If the current token it's the expected token return true, else return false
    fn peek_expect(&mut self, expected: &TokenType) -> bool {
        self.peek_type().eq(expected)
    }

    fn expected_error(&self, expected: &str, found: &Token) {
        notify::Message {
            text: String::from(format!(
                "expected {} but found {}",
                expected, found.token_value
            )),
            line: self.current().line,
            column: self.current().column,
        }
        .show_error();
        exit(1);
    }

    fn expected_or_error(&mut self, expected_type: &TokenType, expected: &str, found: &Token) {
        match eq(self.current_type(), expected_type) {
            true => {
                notify::Message {
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
            false => {}
        }
    }

    fn get_type(&mut self) -> Types {
        match self.current_type() {
            TokenType::Int => Types::Int,
            TokenType::Float => Types::Float,
            TokenType::Bool => Types::Bool,
            TokenType::Char => Types::Char,
            TokenType::Str => Types::Str,
            TokenType::Void => Types::Void,
            TokenType::Any => Types::Any,
            _ => Types::Any,
        }
    }

    fn parse_params(&mut self) -> Vec<FuncParam> {
        // '('
        self.advance();

        // "name: type" <- without the space
        let mut param: FuncParam;
        let mut params: Vec<FuncParam> = Vec::new();

        // If doesn't have parameters
        if self.current_type().eq(&TokenType::RParen) {
            return params;
        }

        // while doesn't reaches ')' or EOF
        while !self.peek_expect(&TokenType::RParen) || !self.peek_expect(&TokenType::Eof) {
            // If the first piece of the param isn't a identifier (name)
            if !self.current_type().eq(&TokenType::Identifier) {
                self.expected_error("identifier", self.current());
                exit(1);
            }

            let name = self.current().to_owned().token_value;
            self.advance();

            // After the name ':'
            self.expected_or_error(&TokenType::Colon, ":", &self.current().to_owned());
            self.advance();

            // The type of the parameter
            let r#type: Types = self.get_type();
            self.advance();

            param = FuncParam { name, r#type };

            if self.current_type().eq(&TokenType::RParen) {
                params.push(param.to_owned());
                break;
            } else if self.current_type().eq(&TokenType::Comma) {
                params.push(param.to_owned());
                self.advance();
            } else {
                self.expected_error(", or )", self.current());
                exit(1);
            }
        }

        if !self.current_type().eq(&TokenType::RParen) {
            self.expected_error(")", self.current());
            exit(1);
        }

        return params;
    }

    fn parse_block(&mut self, is_loop: &Loop) -> Option<Vec<Statement>> {
        // '{' <- The start of the block
        self.advance();

        // Block content
        let mut block_statements: Vec<Statement> = Vec::new();
        let mut statement: Statement;

        while !self.current_type().eq(&TokenType::RBracket)
            || !self.current_type().eq(&TokenType::Eof)
        {
            statement = match self.current_type() {
                TokenType::KwVar => {
                    let declaration = self.parse_var_statement();
                    dbg!(&declaration);

                    declaration
                }
                _ => exit(1),
            }
        }

        // '}' <- The end of the block
        self.expected_or_error(&TokenType::RBracket, "}", &self.current().to_owned());

        if block_statements.len() == 0 {
            return None;
        } else {
            return Some(block_statements);
        }
    }

    /// Parse and return a statement of function
    // func name(arg1: type, arg2: type) -> type {}
    fn parse_function_statement(&mut self) {
        // "func" <- Token
        let func_token = self.current().to_owned();
        // self.advance();

        // "name" <- The function identifier
        self.expected_or_error(
            &TokenType::Identifier,
            "identifier",
            &self.current().to_owned(),
        );
        let name = self.current().token_value.to_owned();
        self.advance();

        // '(' <- Start of the parameters after the function identifier
        self.expected_or_error(&TokenType::LParen, "(", &self.current().to_owned());
        self.advance();

        // The function parameters
        let params: Vec<FuncParam> = self.parse_params();

        // ')' <- The end of function parameters
        self.advance();

        // The type of the function
        let r#type: Types;

        // '->' <- To define the function return type
        // The type is optional
        if !self.peek_expect(&TokenType::OpSetOrAcess) {
            self.advance();
            r#type = self.get_type();
        } else {
            // self.advance();
            r#type = Types::Any;
        }

        // '{' <- The start of the code block of the function
        self.expected_or_error(&TokenType::LBrace, "{", &self.current().to_owned());
        self.advance();

        let function_body: Option<Vec<Statement>> = self.parse_block(&Loop::No);
    }

    fn parse_var_statement(&mut self) -> Statement {
        // "var" <- The start of the declaration
        let var_token = self.current().clone();

        self.advance();

        // "name" <- The identifier of the variable
        self.expected_or_error(
            &TokenType::Identifier,
            "identifier",
            &self.current().to_owned(),
        );

        let name = self.current().token_value.to_owned();
        self.advance();

        // The type of the variable
        let r#type: Types;

        // ':' <- After the variable identifier to define the variable type
        // The type is optional
        if !self.peek_expect(&TokenType::Colon) {
            self.advance();
            r#type = self.get_type(); // ...: type ...
            self.advance();
        } else {
            r#type = Types::Any;
        }

        // ';' or a operator <- before the name or type of the variable
        if !self.peek_expect(&TokenType::SemiColon) {
            let var_declaration = Statement::VariableDeclaration {
                start: Position {
                    line: var_token.line,
                    column: var_token.column,
                },
                name: name.to_owned(),
                r#type,
                value: None,
            };

            return var_declaration;
        } else {
            match self.current_type() {
                &TokenType::OpAssign => {}
                &TokenType::OpInferredTypeAssing => {}
                _ => self.expected_error("'=' or ':='", self.current()),
            }
        }

        // The final variable declaration statement result
        let var_declaration = Statement::VariableDeclaration {
            start: Position {
                line: var_token.line,
                column: var_token.column,
            },
            name: String::from("name"),
            r#type: Types::Any,
            value: None,
        };

        return var_declaration;
    }
}
