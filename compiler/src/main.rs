use std::env;
use std::fs;
use colored::Colorize;
use commom;

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
        commom::show_help_content("compiler");
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
                    
                    let source = read_source(&source_path);
                    println!("{}", source);
                }
            }
        }
    }
}

fn read_source(file_path: &String) -> String {    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents;
}
