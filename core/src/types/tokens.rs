use logos::{Lexer, Logos};

// All tokens of the language
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = (usize, usize))]
pub enum TokenType {
    // Special symbols
    #[token("\n", newline_callback)]
    NewLine,

    #[regex(r"[ \t]", word_callback)]
    Space,

    // Punctuation
    #[token(".", word_callback)]
    Dot,

    #[token(",", word_callback)]
    Comma,

    #[token(":", word_callback)]
    Colon,

    #[token(";", word_callback)]
    SemiColon,

    #[token("\'", word_callback)]
    SingleQuote,

    #[token("\"", word_callback)]
    DoubleQuotes,

    // Delimiters
    #[token("(", word_callback)]
    LParen,

    #[token(")", word_callback)]
    RParen,

    #[token("[", word_callback)]
    LBrace,

    #[token("]", word_callback)]
    RBrace,

    #[token("{", word_callback)]
    LBracket,

    #[token("}", word_callback)]
    RBracket,

    #[token("-#", word_callback)]
    OpenComment,

    #[token("#-", word_callback)]
    CloseComment,

    // Declaration keywords
    #[token("import", word_callback)]
    KwImport,

    #[token("var", word_callback)]
    KwVar,

    #[token("const", word_callback)]
    KwConst,

    #[token("class", word_callback)]
    KwClass,

    #[token("struct", word_callback)]
    KwStruct,

    #[token("func", word_callback)]
    KwFunc,

    // Logical Keywords
    #[token("if", word_callback)]
    KwIf,

    #[token("else", word_callback)]
    KwElse,

    #[token("elif", word_callback)]
    KwElif,

    // Loops keywords
    #[token("loop", word_callback)]
    KwLoop,

    #[token("while", word_callback)]
    KwWhile,

    #[token("for", word_callback)]
    KwFor,

    #[token("continue", word_callback)]
    KwContinue,

    #[token("break", word_callback)]
    KwBreak,

    #[token("return", word_callback)]
    KwReturn,

    // Built-in types
    #[token("null", word_callback)]
    Null,

    #[token("int", word_callback)]
    Int,

    #[token("float", word_callback)]
    Float,

    #[token("bool", word_callback)]
    Bool,

    #[token("str", word_callback)]
    Str,

    #[token("void", word_callback)]
    Void,

    #[token("any", word_callback)]
    Any,

    // Binary Operators
    #[token("+", word_callback)]
    OpPlus,

    #[token("-", word_callback)]
    OpMinus,

    #[token("*", word_callback)]
    OpAsterisk,

    #[token("/", word_callback)]
    OpDivision,

    #[token("%", word_callback)]
    OpRest,

    // Logical operators
    #[token("!", word_callback)]
    #[token("not", word_callback)]
    OpNot,

    #[token("&", word_callback)]
    #[token("and", word_callback)]
    OpAnd,

    #[token("|", word_callback)]
    #[token("or", word_callback)]
    OpOr,

    // Logical ternary -> (true or false)
    #[token("<", word_callback)]
    OpSmallerThan,

    #[token(">", word_callback)]
    OpGreaterThan,

    #[token(">=", word_callback)]
    OpSmallerOrEqualsThan,

    #[token("<=", word_callback)]
    OpGreaterOrEqualsThan,

    #[token("==", word_callback)]
    OpEquals,

    #[token("!=", word_callback)]
    OpNotEquals,

    // Assignment operators
    #[token("=", word_callback)]
    OpAssign,

    #[token("+=", word_callback)]
    OpAssignPlus,

    #[token("-=", word_callback)]
    OpAssignMinus,

    #[token("*=", word_callback)]
    OpAssignMultiply,

    #[token("/=", word_callback)]
    OpAssignDivision,

    #[token("%=", word_callback)]
    OpAssignRest,

    // Special operators -> Assignment or acess
    #[token("->", word_callback)]
    OpSetOrAcess,

    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", word_callback)]
    Identifier,

    #[regex("[0-9]+")]
    Number,
}

// A token struct who stores the token type, value and the position of the token
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub line: usize,
    pub column: usize,
}

// A struct that stores a position -> (line and column)
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Update the line count and the char index.
fn newline_callback(lex: &mut Lexer<TokenType>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

/// Compute the line and column position for the current word.
fn word_callback(lex: &mut Lexer<TokenType>) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;
}
