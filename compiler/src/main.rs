use std::env;

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
    if args.len() < 2 {
        help_content();
    } else {
        for arg in args {    
            if arg.chars().nth(0).unwrap() == '-' {
                match arg.as_str() {
                    "-werror" => flags.werror = true,
                    _ => println!("{}: Non existent argument", arg)
                }
            } else {
                println!("{}: source file.", arg);
            }
        }
    }
}

// TODO: Make a better help mensage for the users
fn help_content() {
    println!("Help content comming soon!");
}
