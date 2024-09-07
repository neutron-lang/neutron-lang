//enum Token {
//    SEMICOLON
//}

pub fn lexer_source(source: &String) {
    for a in source_to_array(&source) {
        println!("[{}]", a);
    }
}

fn source_to_array(source: &String) -> Vec<String> {
    // All the symbols and operators used for mark the division of the words
    let special_symbols = vec!["(", ")", "{", "}", ";", ",", ".", r#"""#];
    let operators = vec!["=", "+", "-", "*", "/", "<", ">"];
    
    let mut current_text = String::new(); // current_text: retains the current word formed by the c in the for loop below.
    let mut result_array = vec![];   // result_array: retains the result of the function, the for loop below insert the current_text here before the current_text clear.

    // The c variable of the loop retains the current char of the source, if it's a special symbol or operator, it's inserted in the result_array, else, it is inserted in the current word.
    for c in source.chars() {
        if c == ' ' || c == '\n' || special_symbols.contains(&c.to_string().as_str()) || operators.contains(&c.to_string().as_str()) {
            if current_text != ""{
                result_array.insert(result_array.len(), current_text.clone());
            }
            
            if c != '\n' {
                result_array.insert(result_array.len(), c.to_string());
            }
            
            current_text.clear()
        } else {
            current_text.push(c);
        }
    }
    
    return result_array;
}