//enum Token {
//    SEMICOLON
//}

pub fn lexer_source(source: &String) {
    source_to_array(&source);
}

fn source_to_array(source: &String) {
    let symbols = vec![" ", "(", ")", "{", "}", ";", r#"""#];
    let mut current_word = String::new();
    let mut current_array: &Vec<String>;

    for c in source.chars() {
        if symbols.contains(&c.to_string().as_str()) {
            println!("{}: symbol", c);
        }
    }
}
