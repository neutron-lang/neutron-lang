use crate::lexer::Token;
use crate::notify::Message;

pub fn verify_lexer(result: &Vec<Token>, file_name: &String) {
    let mut message = Message {
        file: file_name.to_string(),
        text: String::new(),
        line: 0,
        column: 0
    };
    
    let mut expect: Token;
    
    for token in result {
        match token {
            Token::Keyword(values) => {
                message.text = String::from(&values.2);
                message.line = values.0.clone();
                message.column = values.1.clone();
                
                message.show_error();
            },
            _ => message.show_warning()
        }
    }
}