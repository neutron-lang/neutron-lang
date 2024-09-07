use std::env;
use colored::Colorize;
use commom;

mod lexer;

struct Flags {
    werror: bool
}

impl Flags {
    fn set_flag(&mut self, flag_id: &str) {
        match flag_id {
            "-werror" => self.werror = true,

            _ => {
                println!("{}: {} non existent argument", "Error".red(), flag_id);
                std::process::exit(0);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut compiler_flags = Flags {
        werror: false
    };
    
    parse_args(&args, &mut compiler_flags);
}

fn parse_args(args: &Vec<String>, flags: &mut Flags) {
    // If doesn't receive arguments, so print on the console the help content of the compiler
    if args.len() < 2 {
        commom::show_help_content("compiler",
            env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"), 
            env!("CARGO_PKG_DESCRIPTION"));
    } else {
        for arg in args {
            // If the argument isn't the first argument, so can parse it.
            if &args[0] != arg {
                // If the argument start with "-", so it is a compiler flag.
                if arg.chars().nth(0).unwrap() == '-' {
                    flags.set_flag(arg.as_str())
                } else { // Else the argument is a source file, a file with sol code.
                    let cpath = env::current_dir().unwrap();
                    let source_path = cpath.into_os_string().into_string().unwrap()+ "/" + arg;
                    
                    let source = commom::read_source(&source_path);
                    lexer::lexer_source(&source);
                }
            }
        }
    }
}
