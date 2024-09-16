use crate::notify::Message;
use logos::{Lexer, Logos};
use std::process;

// All tokens of the language
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = (usize, usize, String))]
pub enum Token {
    #[token("(", token_callback)]
    LeftParenthesis((usize, usize, String)),
    #[token(")", token_callback)]
    RigthParenthesis((usize, usize, String)),

    #[token("[", token_callback)]
    LeftBrace((usize, usize, String)),
    #[token("]", token_callback)]
    RigthBrace((usize, usize, String)),

    #[token("{", token_callback)]
    LeftBracket((usize, usize, String)),
    #[token("}", token_callback)]
    RigthBracket((usize, usize, String)),

    #[token("-#", token_callback)]
    OpenComment((usize, usize, String)),

    #[token("#-", token_callback)]
    CloseComment((usize, usize, String)),

    #[token(".", token_callback)]
    Period((usize, usize, String)),

    #[token(",", token_callback)]
    Comma((usize, usize, String)),

    #[token(":", token_callback)]
    Column((usize, usize, String)),

    #[token(";", token_callback)]
    SemiColumn((usize, usize, String)),

    #[token(r#"'"#, token_callback)]
    SingleQuote((usize, usize, String)),
    #[token(r#"""#, token_callback)]
    DoubleQuotes((usize, usize, String)),

    #[regex(r"\n", newline_callback)]
    NewLine,

    #[token(" ", token_callback)]
    Space((usize, usize, String)),

    #[token("import", token_callback)]
    #[token("func", token_callback)]
    #[token("var", token_callback)]
    #[token("if", token_callback)]
    #[token("else", token_callback)]
    #[token("class", token_callback)]
    #[token("struct", token_callback)]
    #[token("int", token_callback)]
    #[token("float", token_callback)]
    #[token("bool", token_callback)]
    #[token("str", token_callback)]
    #[token("void", token_callback)]
    Keyword((usize, usize, String)),

    #[regex("[a-zA-Z0-9_]+", token_callback)]
    Identifier((usize, usize, String)),

    #[token("=", token_callback)]
    #[token("+", token_callback)]
    #[token("-", token_callback)]
    #[token("*", token_callback)]
    #[token("/", token_callback)]
    #[token("%", token_callback)]
    #[token("<", token_callback)]
    #[token(">", token_callback)]
    #[token("!", token_callback)]
    #[token("+=", token_callback)]
    #[token("-=", token_callback)]
    #[token("*=", token_callback)]
    #[token("/=", token_callback)]
    #[token("%=", token_callback)]
    #[token("==", token_callback)]
    #[token("!=", token_callback)]
    #[token(">=", token_callback)]
    #[token("<=", token_callback)]
    #[token("->", token_callback)]
    Operator((usize, usize, String)),

    #[regex("[0-9]+", priority = 3, callback = token_callback)]
    Number((usize, usize, String)),
}

fn token_callback(lex: &mut Lexer<Token>) -> (usize, usize, String) {
    let value = lex.slice();
    let position = position_callback(lex);

    (position.0, position.1, value.to_string())
}

fn newline_callback(lex: &mut Lexer<Token>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

// Compute the line and column position for the current word.
fn position_callback(lex: &mut Lexer<Token>) -> (usize, usize) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;

    (line, column)
}

impl Token {
    pub fn lex_trim(result: &Vec<Token>) -> Vec<Token> {
        let mut in_string: bool = false;
        let mut trim_result = vec![];

        for token in result {
            if in_string {
                match token {
                    Token::DoubleQuotes(_) => {
                        in_string = false;
                        trim_result.insert(trim_result.len(), token.clone());
                    }
                    _ => trim_result.insert(trim_result.len(), token.clone()),
                }
            } else {
                match token {
                    Token::DoubleQuotes(_) => {
                        in_string = true;
                        trim_result.insert(trim_result.len(), token.clone());
                    }
                    Token::NewLine => continue,
                    Token::Space(_) => continue,
                    _ => trim_result.insert(trim_result.len(), token.clone()),
                }
            }
        }

        return trim_result;
    }
}

pub fn lex_source(source: &String) -> Vec<Token> {
    let mut lex = Token::lexer(source);
    let mut lexer_result = vec![];
    let mut message = Message {
        text: String::new(),
        line: 0,
        column: 0,
    };
    let mut error_count = 0;
    let mut can_proceed = true;

    while let Some(result) = lex.next() {
        match result {
            Ok(token) => lexer_result.insert(lexer_result.len(), token),
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

    if can_proceed {
        lexer_result = Token::lex_trim(&lexer_result);
    } else {
        message.text = String::from(format!("Can't proceed due by {} errors.", error_count));
        message.show_message("compiler".to_string());
        process::exit(1);
    }

    // dbg!(&lexer_result);
    return lexer_result;
}
