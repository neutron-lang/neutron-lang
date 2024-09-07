use std::env;
use commom;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
}

fn parse_args(args: &Vec<String>) {
    if args.len() < 2 {
        commom::show_help_content("cli",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"), 
            env!("CARGO_PKG_DESCRIPTION"));
    } else {
        for arg in args {    
            if arg.chars().nth(0).unwrap() == '-' {
                println!("{}: argument.", arg);
            } else {
                println!("{}: source file.", arg);
            }
        }
    }
}
