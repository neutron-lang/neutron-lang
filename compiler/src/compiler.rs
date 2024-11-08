use core::error_handler::*;

#[derive(Debug)]
pub struct Flags {
    werror: bool,
    output: String,
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            werror: false,
            output: String::from("a.out"),
        }
    }

    pub fn set_flag(&mut self, flag_id: &str) {
        match flag_id {
            "-werror" => self.werror = true,

            _ => invalid_compiler_flag(flag_id),
        }
    }
}
