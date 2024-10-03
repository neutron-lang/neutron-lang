use colored::Colorize;
use std::fs;
use std::process;

pub mod error_handler;
pub mod frontend;
pub mod notify;
pub mod types;

/// Lex and parse the input file, and return a bytecode
pub fn analyze_source(file_path: &String, file_name: &String) -> frontend::parser::Parser {
    // The source code
    let source = read_source(file_path, file_name);

    // Lex the code and returns a vec of tokens
    let lexer_result = frontend::lexer::lex_source(&source);

    // Parse the lexer result and returns a ast
    let mut parser_result = frontend::parser::Parser::new(lexer_result);
    parser_result.parse_tokens();
    // dbg!(&parser_result);

    return parser_result;
}

// TODO: Make a better help mensage for the users
pub fn show_help_content(from: &str, description: &str) {
    println!("{}\n", description);

    match from {
        "cli" => {
            println!(
                "{} - Compile and link.\n{} - Run the bin.",
                "Build".yellow(),
                "Run".yellow()
            )
        }
        "compiler" => println!("Help content comming soon!"),
        "linker" => println!("Help content comming soon!"),
        _ => println!("{}: Witch help information?", "Error".red()),
    }

    println!("");
}

/// Read the content of the input file
pub fn read_source(file_path: &String, file_name: &String) -> String {
    let contents = match fs::read_to_string(file_path) {
        Ok(value) => value,
        Err(e) => {
            notify::Message {
                text: String::from(format!("{} -> {:?}", file_name, e.kind())),
                line: 0,
                column: 0,
            }
            .show_message("neutron".to_string());
            process::exit(1);
        }
    };

    return contents;
}
