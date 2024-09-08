pub fn get_unit_operators<'a>() -> Vec<&'a str> {
    let unit_operators = vec!["=", "+", "-", "/", "*", "%", "<", ">", "#"];
    
    return unit_operators;
}

pub fn get_binary_operators<'a>() -> Vec<&'a str> {
    let binary_operators = vec!["==", "+=", "-=", "*=", "/=", "%=", "<=", ">=", "->"];
    
    return binary_operators;
}

pub fn get_keywords<'a>() -> Vec<&'a str> {
    let keywords = vec!["import", "func", "var", "if", "else", "elif", "while", "for", "match", "when", "break", "continue", "return"];
    
    return keywords;
}

pub fn get_special_symbols<'a>() -> Vec<&'a str> {
    let special_symbols = vec!["(", ")", "{", "}", "[", "]", ".", ",", ":", ";", r#"""#, r#"'"#, r#"\"#];
    
    return special_symbols;
}