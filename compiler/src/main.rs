use core::{self, analyze_source, error_handler::*};
use std::{env, iter::Peekable};
mod compiler;

#[derive(Debug)]
struct Arguments {
    args: Peekable<std::vec::IntoIter<String>>,
    current_arg: String,
    flags: compiler::Flags,
}

impl Arguments {
    fn new(input: Vec<String>) -> Self {
        let input_iter = input.into_iter().peekable();

        Self {
            current_arg: input_iter.clone().next().unwrap(),
            args: input_iter,
            flags: compiler::Flags::new(),
        }
    }

    fn current(&self) -> &String {
        &self.current_arg
    }

    fn advance(&mut self) -> usize {
        match self.args.peek() {
            Some(_) => {
                self.args.next();
                return 0;
            }
            None => {
                return 1;
            }
        }
    }
}

fn main() {
    // Collect the arguments from the cmd, and remove the first argument
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut cmd_args = Arguments::new(args);

    // Parse the arguments
    parse_args(&mut cmd_args);
}

fn parse_args(cmd_args: &mut Arguments) {
    // Turn the arguments into a peekable iterator
    let mut index: usize = 0;

    // If doesn't receive arguments, so print the help content of the compiler on the console.
    if cmd_args.args.len() < 1 {
        core::show_help_content("compiler", env!("CARGO_PKG_DESCRIPTION"));
        return;
    }

    // Else while the index is less than the argument count
    while index < cmd_args.args.len() {
        // If the argument start with '-', so it's a compiler flag.
        if cmd_args.current().chars().nth(0).unwrap() == '-' {
            println!("arg");
        } else {
            println!("value");
        }

        if cmd_args.advance() == 0 {
            println!("ok");
        }

        index += 1;
    }
}
