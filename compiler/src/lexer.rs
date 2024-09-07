//enum Token {
//    SEMICOLON
//}

pub fn lexer_source(source: &String) {
    let mut current_word = String::new();
    
    for c in source.chars() {
        if c != ' ' {
            current_word.push_str(&c.to_string());
        } else {
            current_word.clear()
        }
        
        println!("{}", current_word);
    }
}