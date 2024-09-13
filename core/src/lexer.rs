use logos::{Lexer, Logos};

// All tokens of the language
#[derive(Logos, Debug)]
#[logos(extras = (usize, usize, String))]
pub enum TokenType {
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
    Keyword((usize, usize, String)),
    
    #[regex("[a-zA-Z]+", token_callback)]
    Identifier((usize, usize, String)),
    
    #[token("=", token_callback)]
    #[token("+", token_callback)]
    #[token("-", token_callback)]
    #[token("*", token_callback)]
    #[token("/", token_callback)]
    #[token("%", token_callback)]
    #[token("+=", token_callback)]
    #[token("-=", token_callback)]
    #[token("*=", token_callback)]
    #[token("/=", token_callback)]
    #[token("%=", token_callback)]
    #[token("==", token_callback)]
    #[token("!=", token_callback)]
    #[token(">=", token_callback)]
    #[token("<=", token_callback)]
    Operator((usize, usize, String)),
    
    #[regex("[0-9]+", token_callback)]
    Number((usize, usize, String))
}

fn token_callback(lex: &mut Lexer<TokenType>) -> (usize, usize, String) {
    let value = lex.slice();
    let position = position_callback(lex);
    
    (position.0, position.1, value.to_string())
}

fn newline_callback(lex: &mut Lexer<TokenType>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

// Compute the line and column position for the current word.
fn position_callback(lex: &mut Lexer<TokenType>) -> (usize, usize) {
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
