// All tokens of the language
pub enum Token {
    // Special symbols.
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftArrow,
    RightArrow,
    Dot,
    Comma,
    Column,
    SemiColumn,
    // StartComment,
    // EndComment,
    Space,
    
    // Simple operators and mixed operators.
    Operator(String),
    
    // Indentifiers, like: names of functions, variables...
    Indentifier(String),
}

pub fn lexer_source(source: &String) -> Vec<Token> {
    let mut token_array = vec![];
    
    for a in source_to_array(&source) {
        let result = tokenizer(&a);
        
        token_array.insert(token_array.len(), result);
    }
    
    for t in &token_array {
        match t {
            Token::LeftParenthesis => println!("LeftParenthesis"),
            Token::RightParenthesis => println!("RightParenthesis"),
            Token::LeftBrace => println!("LeftBrace"),
            Token::RightBrace => println!("RightBrace"),
            Token::LeftBracket => println!("LeftBracket"),
            Token::RightBracket => println!("RightBracket"),
            Token::LeftArrow => println!("LeftArrow"),
            Token::RightArrow => println!("RightArrow"),
            Token::Dot => println!("Dot"),
            Token::Comma => println!("Comma"),
            Token::Column => println!("Column"),
            Token::SemiColumn => println!("SemiColumn"),
            Token::Space => println!("Space"),
            Token::Operator(_) => println!("Operator"),
            Token::Indentifier(_) => println!("Indentifier"),
        }
    }
    
    return token_array;
}

fn source_to_array(source: &String) -> Vec<String> {
    // All the symbols and operators used for mark the division of the words
    let special_symbols = vec!["(", ")", "{", "}", ";", ",", ".", r#"""#];
    let operators = vec!["=", "+", "-", "*", "/", "<", ">"];
    
    let mut current_text = String::new(); // current_text: retains the current word formed by the c in the for loop below.
    let mut result_array = vec![];   // result_array: retains the result of the function, the for loop below insert current_text here before current_text clear.

    // The c variable of the loop retains the current char of the source, if it's a special symbol or operator, it's inserted in result_array, else, it is inserted in current_word.
    for c in source.chars() {
        if c == ' ' || c == '\n' || special_symbols.contains(&c.to_string().as_str()) || operators.contains(&c.to_string().as_str()) {
            // If current_text is diferent of nothing, so insert it in result_array.
            if current_text != ""{
                result_array.insert(result_array.len(), current_text.clone());
            }
            
            // If the current char is a \n, ignore it, else insert it in current_array
            if c != '\n' {
                result_array.insert(result_array.len(), c.to_string());
            }
            
            // Clear current_text
            current_text.clear()
        } else {
            // Insert the current char in current_text
            current_text.push(c);
        }
    }
    
    return result_array;
}

fn tokenizer(input: &String) -> Token {
    match input.as_str() {
        "(" => return Token::LeftParenthesis,
        ")" => return Token::RightParenthesis,
        "[" => return Token::LeftBrace,
        "]" => return Token::RightBrace,
        "{" => return Token::LeftBracket,
        "}" => return Token::RightBracket,
        "<" => return Token::LeftArrow,
        ">" => return Token::RightArrow,
        "." => return Token::Dot,
        "," => return Token::Comma,
        ":" => return Token::Column,
        ";" => return Token::SemiColumn,
        " " => return Token::Space,
        "=" => Token::Operator(input.to_string()),
        "+" => Token::Operator(input.to_string()),
        "-" => Token::Operator(input.to_string()),
        "*" => Token::Operator(input.to_string()),
        "/" => Token::Operator(input.to_string()),
         _  => return Token::Indentifier(input.to_string())
    }
}
