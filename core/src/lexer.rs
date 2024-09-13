use logos::{Lexer, Logos};

// All tokens of the language
#[derive(Logos, Debug)]
#[logos(extras = (usize, usize))]
pub enum TokenType {
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RigthParenthesis,
    
    #[token("[")]
    LeftBrace,
    #[token("]")]
    RigthBrace,
    
    #[token("{")]
    LeftBracket,
    #[token("}")]
    RigthBracket,
    
    #[token(".")]
    Period,
    
    #[token(",")]
    Comma,
    
    #[token(":")]
    Column,
    
    #[token(";")]
    SemiColumn,
    
    #[token(r#"'"#)]
    SingleQuote,
    #[token(r#"""#)]
    DoubleQuotes,
    
    #[regex(r"\n", newline_callback)]
    NewLine,
    
    #[token(" ")]
    Space,
    
    #[token("import", word_callback)]
    #[token("func", word_callback)]
    #[token("var", word_callback)]
    Keyword((usize, usize)),
    
    #[regex("[a-zA-Z]+", word_callback)]
    Identifier((usize, usize)),
}

fn newline_callback(lex: &mut Lexer<TokenType>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

/// Compute the line and column position for the current word.
fn word_callback(lex: &mut Lexer<TokenType>) -> (usize, usize) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;

    (line, column)
}

pub fn lex_source(source: &String){
    let mut lex = TokenType::lexer(source);

    while let Some(token) = lex.next() {
        dbg!(&token);
    }
}
