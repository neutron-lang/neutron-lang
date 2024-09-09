use crate::language_grammar::{get_binary_operators, get_keywords, get_special_symbols, get_unit_operators};

// All tokens of the language
#[derive(Debug)]
pub enum TokenType {
    // Special symbols.
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Column,
    SemiColumn,
    SingleQuote,
    DoubleQuotes,
    EscapeSequence,
    // StartComment,
    // EndComment,
    Space,
        
    // Simple operators and mixed operators.
    Operator,
    
    // Indentifiers, like: names of functions, variables...
    Indentifier,
    
    // Keywords, like: func, var, if ...
    Keyword
}

// A struct that armazenates the token value and his type.
#[derive(Debug)]
pub struct Token {
    value: String,
    token_type: TokenType
}

pub fn lexer_source(source: &String) -> Vec<Token> {
    let mut token_vector = vec![];
    
    for a in source_to_vector(&source) {
        let result = tokenizer(&a);
        
        token_vector.insert(token_vector.len(), result);
    }
    
    dbg!(&token_vector);
    
    return token_vector;
}

// Transform the source into a string vector
fn source_to_vector(source: &String) -> Vec<String> {
    // All the symbols and operators used for mark the division of the words
    let special_symbols = get_special_symbols();
    let operators = get_unit_operators();
    
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
    let operators = get_unit_operators();
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
            } else if value == r#"\"# {
                let next_opt = input.get(index + 1);
                let mut next = String::new();
                
                match next_opt {
                    Some(v) => next.push_str(v),
                    _ => next.push_str("null")
                }
                
                let escape_sequence = value.to_string() + next.as_str();
                
                result.insert(result.len(), escape_sequence);
                skip_value = true;
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
    let unit_operators = get_unit_operators();
    let binary_operators = get_binary_operators();
    let special_symbols = get_special_symbols();
    let keywords = get_keywords();
    
    let mut current_token = Token {
        value: input.to_string(),
        token_type: TokenType::Indentifier
    };
    
    if unit_operators.contains(&input.as_str()) || binary_operators.contains(&input.as_str()) {
            current_token.token_type = TokenType::Operator;
        } else if special_symbols.contains(&input.as_str()) {
            if input == "(" {
                current_token.token_type = TokenType::LeftParenthesis;
            } else if input == ")" {
                current_token.token_type = TokenType::RightParenthesis;
            } else if input == "[" {
                current_token.token_type = TokenType::LeftBrace;
            } else if input == "]" {
                current_token.token_type = TokenType::RightBrace;
            } else if input == "{" {
                current_token.token_type = TokenType::LeftBracket;
            } else if input == "}" {
                current_token.token_type = TokenType::RightBracket;
            } else if input == "." {
                current_token.token_type = TokenType::Dot;
            } else if input == "," {
                current_token.token_type = TokenType::Comma;
            } else if input == ":" {
                current_token.token_type = TokenType::Column;
            } else if input == ";" {
                current_token.token_type = TokenType::SemiColumn;
            } else if input == r#""# {
                current_token.token_type = TokenType::SingleQuote;
            } else {
                current_token.token_type = TokenType::DoubleQuotes;
            }
        } else if input == " " {
            current_token.token_type = TokenType::Space;
        } else if keywords.contains(&input.as_str()) {
            current_token.token_type = TokenType::Keyword;
        } else if input.chars().nth(0).unwrap() == '\\' {
            current_token.token_type = TokenType::EscapeSequence;
        } else {
            current_token.token_type = TokenType::Indentifier;
        }
    
    return current_token;
}
