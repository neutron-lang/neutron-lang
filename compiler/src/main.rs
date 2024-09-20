use core::{self, notify};
use std::env;

struct Flags {
    werror: bool,
}

impl Flags {
    fn set_flag(&mut self, flag_id: &str) {
        match flag_id {
            "-werror" => self.werror = true,

            _ => {
                notify::Message {
                    text: format!("{} -> non existent flag.", flag_id),
                    line: 0,
                    column: 0,
                }
                .show_message("neutron".to_string());
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut compiler_flags = Flags { werror: false };

    parse_args(&args, &mut compiler_flags);
}

fn parse_args(args: &Vec<String>, flags: &mut Flags) {
    // If doesn't receive arguments, so print on the console the help content of the compiler
    if args.len() < 2 {
        core::show_help_content("compiler", env!("CARGO_PKG_DESCRIPTION"));
    } else {
        for arg in args {
            // If the argument isn't the first argument, so can parse it.
            if &args[0] != arg {
                // If the argument start with "-", so it is a compiler flag.
                if arg.chars().nth(0).unwrap() == '-' {
                    flags.set_flag(arg.as_str())
                } else {
                    // Else the argument is a source file, a file with sol code.
                    let cpath = env::current_dir().unwrap();
                    let source_path = cpath.into_os_string().into_string().unwrap() + "/" + arg;

                    let source = core::read_source(&source_path);

                    notify::Message {
                        text: format!("compiling -> {}", arg),
                        line: 0,
                        column: 0,
                    }
                    .show_message("compiler".to_string());

                    let lexer_result = core::frontend::lexer::lex_source(&source);
                    let parser = core::frontend::parser::Parser::new(lexer_result);
                    dbg!(parser);
                }
            }
        }
    }
}
