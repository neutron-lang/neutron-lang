use logos::{Lexer, Logos};

// All tokens of the language
#[derive(Logos, Debug)]
#[logos(extras = (usize, usize))]
pub enum Token {
    #[regex(r"\n", newline_callback)]
    NewLine,
    
    #[token("import", word_callback)]
    #[token("func", word_callback)]
    Keyword((usize, usize)),
    
    #[regex("[a-zA-Z]+", word_callback)]
    Identifier((usize, usize)),
}

/*
// A struct that armazenates the token value and his type.
#[derive(Debug)]
pub struct Token {
    value: String,
    token_type: TokenType
}
*/

fn newline_callback(lex: &mut Lexer<Token>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

/// Compute the line and column position for the current word.
fn word_callback(lex: &mut Lexer<Token>) -> (usize, usize) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;

    (line, column)
}

pub fn lex_source(source: &String) {
    let mut lex = Token::lexer(source);

    while let Some(token) = lex.next() {
        match token {
            Ok(tp) => {
                match tp {
                    Token::NewLine => println!("Newline found"),
                    Token::Keyword((line, column)) => println!("Keyword '{}' found at ({}, {})", lex.slice(), line, column),
                    Token::Identifier((line, column)) => println!("Indetifier '{}' found at ({}, {})", lex.slice(), line, column)
                }
            },
            Err(e) => println!("Error: {:?}", e)
        }
    }
}
