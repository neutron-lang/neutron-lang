use colored::Colorize;
use std::fs;

pub mod frontend;
pub mod notify;

// TODO: Make a better help mensage for the users
pub fn show_help_content(from: &str, version: &str, description: &str) {
    println!("{} version: .{}\n", description, version.green());

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

// Read the content of the source file
pub fn read_source(file_path: &String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    return contents;
}
