use colored::Colorize;

// TODO: Make a better help mensage for the users
pub fn show_help_content(from: &str) {
    println!("Help content comming soon!");
    
    match from {
        "cli" => {
            println!("{} - Compile and link.\n{} - Run the bin.",
                "Build".yellow(),
                "Run".yellow())
        },
        "compiler" => println!("Help content comming soon!"),
        "linker" => println!("Help content comming soon!"),
        _ => println!("{}: Witch help information?", "Error".red())
    }
}