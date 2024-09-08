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
    let mut token_vector = vec![];
    
    for a in source_to_vector(&source) {
        let result = tokenizer(&a);
        
        token_vector.insert(token_vector.len(), result);
    }
    
    for t in &token_vector {
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
            Token::Operator(value) => println!("Operator({value})"),
            Token::Indentifier(value) => println!("Indentifier({value})"),
        }
    }
    
    return token_vector;
}

// Transform the source into a string vector
fn source_to_vector(source: &String) -> Vec<String> {
    // All the symbols and operators used for mark the division of the words
    let special_symbols = vec!["(", ")", "{", "}", ";", ",", ".", r#"""#];
    let operators = vec!["=", "+", "-", "*", "/", "<", ">"];
    
    let mut current_text = String::new(); // current_text: retains the current word formed by the c in the for loop below.
    let mut result_vector = vec![];   // result_vector: retains the result of the function, the for loop below insert current_text here before current_text clear.

    // The c variable of the loop retains the current char of the source, if it's a special symbol or operator, it's inserted in result_vector, else, it is inserted in current_word.
    for c in source.chars() {
        if c == ' ' || c == '\n' || special_symbols.contains(&c.to_string().as_str()) || operators.contains(&c.to_string().as_str()) {
            // If current_text is diferent of nothing, so insert it in result_vector.
            if current_text != ""{
                result_vector.insert(result_vector.len(), current_text.clone());
            }
            
            // If the current char is a \n, ignore it, else insert it in current_array
            if c != '\n' {
                result_vector.insert(result_vector.len(), c.to_string());
            }
            
            // Clear current_text
            current_text.clear()
        } else {
            // Insert the current char in current_text
            current_text.push(c);
        }
    }
    
    return correct_vector(&result_vector);
}

// Find binary operators separated in the vector, and transform in a single vector unit.
fn correct_vector(input: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    let operators = vec!["=", "+", "-", "*", "/", "<", ">"];
    let mut skip_value: bool = false;
    
    for (index, value) in input.iter().enumerate() {
        if skip_value != true {
            if operators.contains(&value.as_str()) {
                let next_opt = input.get(index + 1);
                let mut next = String::new();
                
                match next_opt {
                    Some(v) => next.push_str(v),
                    _ => next.push_str("null")
                }
                
                if operators.contains(&next.as_str()) {
                    let binary_operator = value.to_string() + next.as_str();
                    
                    result.insert(result.len(), binary_operator);
                    
                    skip_value = true;
                } else {
                    result.insert(result.len(), value.to_string());
                    skip_value = false;
                }
            } else {
                result.insert(result.len(), value.to_string());
            }
        } else {
            skip_value = false;
        }
    }
        
    return result;
}

// Receive a string and transform it in a Token.
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
