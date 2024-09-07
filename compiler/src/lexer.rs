//enum Token {
//    SEMICOLON
//}

pub fn lexer_source(source: &String) {
    source_to_array(&source);
}

fn source_to_array(source: &String) {
    let special_symbols = vec!["(", ")", "{", "}", ";", r#"""#];
    let operators = vec!["=", "+", "-", "*", "/", "<", ">"];
    
    let mut current_word = String::new();
    let mut current_char_idx: usize = 0;
    let mut current_array = vec![];

    for c in source.chars() {
        if special_symbols.contains(&c.to_string().as_str()) || operators.contains(&c.to_string().as_str()) || c == ' ' {
            let last = get_last_char(&source, current_char_idx);
            let next = get_next_char(&source, current_char_idx);
            
            if current_word != "" && current_word != last.to_string(){
                println!("{}", current_word);
                current_array.insert(current_array.len(), current_word.clone());
            }
            
            current_word.clear();
            current_word.push(c);
            
            if c != ' ' {    
                if next != '_' && operators.contains(&next.to_string().as_str()) {
                    current_word.push(next);
                }
            }
            
            println!("{}", current_word);
            current_array.insert(current_array.len(), current_word.clone());
            current_word.clear();
        } else {
            current_word.push(c);
        }
        
        current_char_idx += 1;
    }
    
    for a in current_array {
        print!("[{}]", a);
    }
}

fn get_next_char(source: &String, current_char: usize) -> char {
    if current_char + 1 != source.len() {
        return source.chars().nth(current_char + 1).unwrap();
    } else {
        return '_';
    }
}

fn get_last_char(source: &String, current_char: usize) -> char {
    if current_char - 1 != 0 {
        return source.chars().nth(current_char - 1).unwrap();
    } else {
        return '_';
    }
}
