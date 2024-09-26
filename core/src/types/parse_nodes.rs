use crate::types::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Program {
        start: others::Position,
        body: Box<Vec<Statement>>,
    },
    VariableDeclaration {
        start: others::Position,
        name: Option<String>,
        kind: VarDeclarationKind,
        r#type: bult_in_types::Types,
        value: Expression,
    },
    FuctionDeclaration {
        start: others::Position,
        name: String,
        r#type: bult_in_types::Types,
        params: Option<Vec<FuncParam>>,
        body: Box<Vec<Statement>>,
    },
    If {
        start: others::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    ElseIf {
        start: others::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    Else {
        start: others::Position,
        body: Option<Box<Vec<Statement>>>,
    },
    While {
        start: others::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
    },
    For {
        start: others::Position,
        variable: Option<Box<Statement>>,
        condition: Option<Expression>,
        variable_update: Option<Box<Statement>>,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    Break {
        start: others::Position,
    },
    Continue {
        start: others::Position,
    },
    Return {
        start: others::Position,
        expression: Expression,
    },
    VariableAlteration {
        name: String,
        operator: tokens::TokenType,
        value: Expression,
    },
    FunctionCall(Expression),
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: String,
    pub r#type: bult_in_types::Types,
}

#[derive(Debug, Clone)]
pub enum VarDeclarationKind {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone)]
pub enum ArrayAcess {
    Acess {
        name: String,
        index: Box<Expression>,
    },
    NestedAcess {
        acess: Box<ArrayAcess>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Binary {
        operator: tokens::TokenType,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Logical {
        operator: tokens::TokenType,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: tokens::TokenType,
        operand: Box<Expression>,
    },
    Literal {
        r#type: tokens::TokenType,
        value: String,
    },
    ArrayLiteral {
        elements: Option<Box<Vec<Expression>>>,
    },
    ArrayAcess(ArrayAcess),
    Call {
        name: String,
        arguments: Option<Box<Vec<Expression>>>,
    },
}
