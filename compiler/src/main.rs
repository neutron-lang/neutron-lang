use std::env;
use std::fs;

struct Flags {
    werror: bool
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
        help_content();
    } else {
        for arg in args {
            if &args[0] != arg {
                if arg.chars().nth(0).unwrap() == '-' {
                    match arg.as_str() {
                        "-werror" => flags.werror = true,
                        _ => println!("{}: Non existent argument", arg)
                    }
                } else {
                    let cpath = env::current_dir().unwrap();
                    let source_path = cpath.into_os_string().into_string().unwrap()+ "/" + arg;
                    
                    let source = read_source(&source_path);
                    println!("{}", source);
                }
            }
        }
    }
}

// TODO: Make a better help mensage for the users
fn help_content() {
    println!("Help content comming soon!");
}

fn read_source(file_path: &String) -> String {    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    return contents;
}
