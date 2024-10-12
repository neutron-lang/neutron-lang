use crate::{
    error_handler::*,
    types::{bult_in_types::*, others::*, parse_nodes::*, tokens::*},
};
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current_token: Token,
    pub ast: Statement,
}

impl Parser {
    /// Returns a new parser struct
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

    fn push_statement(&mut self, statement: Statement) {
        match &mut self.ast {
            Statement::Program { body, .. } => {
                body.push(statement);
            }
            _ => {}
        }
    }

    /// Parse the tokens while doesn't reaches the EOF
    pub fn parse_tokens(&mut self) {
        while !self.peek_expect(&TokenType::Eof) {
            let ast_node = match &self.current_type() {
                TokenType::KwVar => self.parse_var_statement(),
                TokenType::KwFunc => self.parse_function_statement(),
                _ => expected_error("a statement", self.current()),
            };

            self.push_statement(ast_node);
            // self.advance();
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

    /// Returns the next token type
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
                expected_error("a type", self.current());
            }
        }
    }

    /// Parse the parameters inside parenthesis -> (param: type, other_param: type)
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
                expected_error("identifier", self.current());
            }

            let name = self.current().token_value.to_owned();
            self.advance();

            // After the name ':'
            expected_or_error(&TokenType::Colon, ":", self.current());
            self.advance();

            // The type of the parameter
            let r#type: Types = self.get_type();
            self.advance();

            param = FuncParam { name, r#type };

            // The end of the parameters or another parameter
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

    /// Parse identifiers -> function calls, push identifier value...
    fn parse_identifier(&mut self) -> Expression {
        match self.current_type() {
            TokenType::Identifier => match self.peek_type() {
                TokenType::LParen => self.parse_function_call(),
                _ => Expression::Identifier(self.current().token_value.to_owned()),
            },
            _ => expected_error("identifier", self.current()),
        }
    }

    /// Parse a function call -> function_identifier(arguments)
    fn parse_function_call(&mut self) -> Expression {
        // The function identifier(name)
        let name = self.current().token_value.to_owned();
        self.advance();

        // '(' <- The start of the arguments
        self.advance();

        // A vector containing the current arguments of the function call
        let mut argument_vec: Vec<Expression> = Vec::new();

        // The current expression
        let mut expression: Expression;

        // While doesn't reaches the ')'
        while !self.peek_expect(&TokenType::RParen) || !self.peek_expect(&TokenType::Eof) {
            // If reaches the ')' <- End of the arguments
            if self.peek_expect(&TokenType::RParen) {
                break;
            }

            // If reaches the EOF before the ')'
            if self.peek_expect(&TokenType::Eof) {
                expected_error("')'", self.current());
            }

            // If reaches ',' starts a new argument
            if self.peek_expect(&TokenType::Comma) {
                self.advance();
                continue;
            }

            // Parse the expression for the current argument
            expression = self.parse_expression();

            // If after the expression, is a ',' or ')', push the current argument and advance
            if self.peek_type().eq(&TokenType::Comma) || self.peek_type().eq(&TokenType::RParen) {
                argument_vec.push(expression);
                self.advance();
            } else {
                self.advance();
                expected_error("',' or ')'", self.current());
            }
        }

        // If the call doesn't have arguments, return a function without arguments ;)
        if argument_vec.len() == 0 {
            return Expression::Call {
                name,
                arguments: None,
            };
        }

        // A full call with arguments
        return Expression::Call {
            name,
            arguments: Some(Box::new(argument_vec)),
        };
    }

    /// Parse code blocks
    fn parse_block(&mut self, is_loop: &Loop) -> Option<Vec<Statement>> {
        // '{' <- The start of the block
        self.advance();

        // Block content
        let mut block_statements: Vec<Statement> = Vec::new();
        let mut statement: Statement;

        while !self.peek_expect(&TokenType::RBracket) || !self.peek_expect(&TokenType::Eof) {
            if self.peek_expect(&TokenType::RBracket) {
                break;
            }

            statement = match self.current_type() {
                TokenType::KwVar => {
                    let declaration = self.parse_var_statement();

                    declaration
                }

                TokenType::Identifier => {
                    let identifier_statement = self.parse_identifier_statement();

                    identifier_statement
                }
                _ => expected_error("a statement", self.current()),
            };

            block_statements.push(statement);
        }

        // '}' <- The end of the block
        expected_or_error(&TokenType::RBracket, "'}'", self.current());

        if block_statements.len() == 0 {
            return None;
        } else {
            return Some(block_statements);
        }
    }

    /// Parsing expressions related function
    fn parse_expression(&mut self) -> Expression {
        self.parse_or_expression()
    }

    /// 'or' or '||' <- Logical or expressions
    fn parse_or_expression(&mut self) -> Expression {
        let mut left = self.parse_and_expression();

        while self.peek_type().eq(&TokenType::OpOr) {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_and_expression();

            left = Expression::Logical {
                operator: operator.token_type,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return left;
    }

    /// 'and' or '&&' <- Logical and expressions
    fn parse_and_expression(&mut self) -> Expression {
        let mut left = self.parse_comparision_expression();

        while self.peek_type().eq(&TokenType::OpAnd) {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_comparision_expression();

            left = Expression::Logical {
                operator: operator.token_type,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return left;
    }

    /// '==' or '!=' <- Comparision expressions
    fn parse_comparision_expression(&mut self) -> Expression {
        let mut left = self.parse_greater_or_smaller_expression();

        while self.peek_type().eq(&TokenType::OpEquals)
            || self.peek_type().eq(&TokenType::OpNotEquals)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_greater_or_smaller_expression();

            left = Expression::Logical {
                operator: operator.token_type,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return left;
    }

    /// '<', '<=', '>' or '>=' <- Size expressions
    fn parse_greater_or_smaller_expression(&mut self) -> Expression {
        let mut left = self.parse_multiplicative_expression();

        while self.peek_type().eq(&TokenType::OpSmallerThan)
            || self.peek_type().eq(&TokenType::OpSmallerOrEqualsThan)
            || self.peek_type().eq(&TokenType::OpGreaterThan)
            || self.peek_type().eq(&TokenType::OpGreaterOrEqualsThan)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_multiplicative_expression();

            left = Expression::Logical {
                operator: operator.token_type,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return left;
    }

    /// '*', '/' or '%' <- Multiplication, division or rest expression
    fn parse_multiplicative_expression(&mut self) -> Expression {
        let mut left = self.parse_plus_or_subtract_expression();

        while self.peek_type().eq(&TokenType::OpMultiply)
            || self.peek_type().eq(&TokenType::OpDivision)
            || self.peek_type().eq(&TokenType::OpRest)
        {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_plus_or_subtract_expression();

            left = Expression::Binary {
                operator: operator.token_type,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return left;
    }

    /// '+' or '-' <- Sum or subratction expression
    fn parse_plus_or_subtract_expression(&mut self) -> Expression {
        let mut left = self.parse_unary_expression();

        while self.peek_type().eq(&TokenType::OpPlus) || self.peek_type().eq(&TokenType::OpMinus) {
            self.advance();

            let operator = self.current().to_owned();
            self.advance();

            let right = self.parse_primary_expression();

            left = Expression::Binary {
                operator: operator.token_type,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return left;
    }

    /// Parse the expression
    fn parse_primary_expression(&mut self) -> Expression {
        let token = self.current().to_owned();

        match token.token_type {
            TokenType::Identifier => self.parse_identifier(),
            TokenType::Number => Expression::Literal {
                r#type: TokenType::Number,
                value: self.current().token_value.to_owned(),
            },
            TokenType::StringLiteral => Expression::Literal {
                r#type: TokenType::StringLiteral,
                value: self.current().token_value.to_owned(),
            },
            TokenType::True | TokenType::False => Expression::Literal {
                r#type: TokenType::Bool,
                value: self.current().token_value.to_owned(),
            },
            TokenType::Eof => {
                expected_error("';'", &token);
            }
            _ => {
                expected_error("expression", &token);
            }
        }
    }

    /// Parse unary expressions
    fn parse_unary_expression(&mut self) -> Expression {
        match self.current_type() {
            TokenType::OpPlus => {
                self.advance();
                self.parse_unary_expression()
            }
            TokenType::OpMinus => {
                self.advance();
                Expression::Unary {
                    operator: TokenType::OpMinus,
                    operand: Box::new(self.parse_unary_expression()),
                }
            }
            TokenType::OpNot => {
                self.advance();
                Expression::Unary {
                    operator: TokenType::OpNot,
                    operand: Box::new(self.parse_unary_expression()),
                }
            }
            _ => self.parse_primary_expression(),
        }
    }

    /// Parse and return a statement of a function -> func name(arg1: type, arg2: type) -> type {function content}
    fn parse_function_statement(&mut self) -> Statement {
        // "func" <- Token
        let func_token = self.current().to_owned();
        self.advance();

        // "name" <- The function name (identifier)
        expected_or_error(&TokenType::Identifier, "identifier", self.current());
        let name = self.current().token_value.to_owned();
        self.advance();

        // '(' <- Start of the parameters after the function identifier
        expected_or_error(&TokenType::LParen, "(", self.current());

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

        let function_body: Option<Vec<Statement>> = self.parse_block(&Loop::No);

        // '}' <- The end of the code block of the function
        self.advance();

        return Statement::FuctionDeclaration {
            start: Position {
                line: func_token.line,
                column: func_token.column,
            },
            name,
            r#type,
            params: if params.len() == 0 {
                Some(params)
            } else {
                None
            },
            body: match function_body {
                Some(body) => Some(Box::new(body)),
                _ => None,
            },
        };
    }

    /// Parse a function call -> function_name(args);
    fn parse_function_call_statement(&mut self) -> Statement {
        // function_name(args)
        let call = Statement::FunctionCall(self.parse_function_call());
        self.advance();

        // ';' < after the call
        expected_or_error(
            &TokenType::SemiColon,
            "the end of statement",
            self.current(),
        );
        self.advance();

        return call;
    }

    fn parse_identifier_statement(&mut self) -> Statement {
        match self.current_type() {
            TokenType::Identifier => match self.peek_type() {
                TokenType::LParen => self.parse_function_call_statement(),
                _ => expected_error("work in progress", self.current()),
            },
            _ => expected_error("identifier", self.current()),
        }
    }

    /// Parse and return a statement of a varibale
    fn parse_var_statement(&mut self) -> Statement {
        // "var" <- Token
        let var_token = self.current().to_owned();
        self.advance();

        // "name" <- The name (identifier) of the variable
        expected_or_error(
            &TokenType::Identifier,
            "variable identifier",
            self.current(),
        );

        let name = self.current().token_value.to_owned();
        self.advance();

        // The type of the variable
        let mut r#type: Types = Types::Any;

        // ':' <- After the variable identifier to define the variable type
        // The type is optional
        if self.peek_expect(&TokenType::Colon) {
            self.advance();
            r#type = self.get_type(); // ...: type ...
            self.advance();
        } else if self.peek_expect(&TokenType::OpAssign)
            || self.peek_expect(&TokenType::OpInferredTypeAssing)
            || self.peek_expect(&TokenType::SemiColon)
        {
        } else {
            expected_error("':', '=' or ':='", self.current());
        }

        // ';' or a operator <- Before the name or type of the variable will declare a unitialized variable
        // '=' or ':=' <- Will initialize the variable
        if self.peek_expect(&TokenType::SemiColon) {
            self.advance();

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
                _ => expected_error("'=' or ':='", self.current()),
            }

            self.advance();
        }

        // The value of the variable is a expression
        expect_expression_or_error(self.current());
        let value = self.parse_expression();

        self.advance();

        // ';' <- In the end
        expected_or_error(&TokenType::SemiColon, ";", self.current());
        self.advance();

        // The final variable declaration statement result
        let var_declaration = Statement::VariableDeclaration {
            start: Position {
                line: var_token.line,
                column: var_token.column,
            },
            name,
            r#type,
            value: Some(value),
        };

        return var_declaration;
    }
}
