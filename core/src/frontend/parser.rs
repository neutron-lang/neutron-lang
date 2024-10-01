use crate::{
    error_handler::*,
    notify,
    types::{bult_in_types::*, others::*, parse_nodes::*, tokens::*},
};
use std::{iter::Peekable, process::exit};

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

    /// Return the current token
    fn current(&self) -> &Token {
        &self.current_token
    }

    /// Return the current token type
    fn current_type(&mut self) -> &TokenType {
        &self.current().token_type
    }

    /// Advance the index in the token vector
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
        self.current_type().eq(expected)
    }

    /// Return the a value type based in the type of the current token
    fn get_type(&mut self) -> Types {
        match self.current_type() {
            TokenType::Int => Types::Int,
            TokenType::Float => Types::Float,
            TokenType::Bool => Types::Bool,
            TokenType::Char => Types::Char,
            TokenType::Str => Types::Str,
            TokenType::Void => Types::Void,
            TokenType::Any => Types::Any,
            _ => {
                expected_error("a type", &self.current().to_owned());
            }
        }
    }

    fn parse_params(&mut self) -> Vec<FuncParam> {
        // '('
        self.advance();

        // "name: type" <- without the space
        let mut param: FuncParam;
        let mut params: Vec<FuncParam> = Vec::new();

        // If doesn't have parameters
        if self.peek_expect(&TokenType::RParen) {
            return params;
        }

        // while doesn't reaches ')' or EOF
        while !self.peek_expect(&TokenType::RParen) || !self.peek_expect(&TokenType::Eof) {
            // If the first piece of the param isn't a identifier (name)
            if !self.peek_expect(&TokenType::Identifier) {
                expected_error("identifier", &self.current().to_owned());
            }

            let name = self.current().token_value.to_owned();
            self.advance();

            // After the name ':'
            expected_or_error(&TokenType::Colon, ":", &self.current().to_owned());
            self.advance();

            // The type of the parameter
            let r#type: Types = self.get_type();
            self.advance();

            param = FuncParam { name, r#type };

            if self.peek_expect(&TokenType::RParen) {
                params.push(param.to_owned());
                break;
            } else if self.peek_expect(&TokenType::Comma) {
                params.push(param.to_owned());
                self.advance();
            } else {
                expected_error(", or )", self.current());
            }
        }

        if !self.peek_expect(&TokenType::RParen) {
            expected_error(")", self.current());
        }

        return params;
    }

    fn parse_block(&mut self, is_loop: &Loop) -> Option<Vec<Statement>> {
        // '{' <- The start of the block
        self.advance();

        // Block content
        let mut block_statements: Vec<Statement> = Vec::new();
        let mut statement: Statement;

        while !self.peek_expect(&TokenType::RBracket) || !self.peek_expect(&TokenType::Eof) {
            statement = match self.current_type() {
                TokenType::KwVar => {
                    let declaration = self.parse_var_statement();
                    dbg!(&declaration);

                    declaration
                }
                TokenType::RBracket => break,
                _ => break,
            }
        }

        // '}' <- The end of the block
        expected_or_error(&TokenType::RBracket, "'}'", self.current());

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
        self.advance();

        // "name" <- The function name (identifier)
        expected_or_error(&TokenType::Identifier, "identifier", self.current());
        let name = self.current().token_value.to_owned();
        self.advance();

        // '(' <- Start of the parameters after the function identifier
        expected_or_error(&TokenType::LParen, "(", &self.current().to_owned());

        // The function parameters
        let params: Vec<FuncParam> = self.parse_params();

        // ')' <- The end of function parameters
        self.advance();

        // The type of the function
        let r#type: Types;

        // '->' <- To define the function return type
        // The type is optional
        if self.peek_expect(&TokenType::OpSetOrAcess) {
            self.advance();
            r#type = self.get_type();
            self.advance();
        } else if self.peek_expect(&TokenType::LBracket) {
            r#type = Types::Any;
        } else {
            expected_error("'->' or '{'", self.current());
        }

        // '{' <- The start of the code block of the function
        expected_or_error(&TokenType::LBracket, "'{'", self.current());
        self.advance();

        let function_body: Option<Vec<Statement>> = self.parse_block(&Loop::No);
    }

    fn parse_var_statement(&mut self) -> Statement {
        // "var" <- Token
        let var_token = self.current().to_owned();
        self.advance();

        // "name" <- The name (identifier) of the variable
        expected_or_error(
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

        // ';' or a operator <- Before the name or type of the variable will declare a unitialized variable
        // '=' or ':=' <- Will initialize the variable
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
                _ => expected_error("'=' or ':='", &self.current().to_owned()),
            }

            self.advance();
        }

        dbg!(self.current().clone());
        expect_expression_or_error(&self.current().to_owned());

        self.advance();

        // ';' <- In the end
        expected_or_error(&TokenType::SemiColon, ";", &self.current().to_owned());

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
