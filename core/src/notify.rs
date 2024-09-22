use colored::Colorize;

pub struct Message {
    pub text: String,
    pub line: usize,
    pub column: usize,
}

impl Message {
    pub fn show_message(&self, from: String) {
        println!("- [{}]: {}", from.green(), self.text);
    }

    pub fn show_error(&self) {
        println!(
            "- [{}]: ({}:{}) -> {}",
            "error".red(),
            self.line + 1,
            self.column + 1,
            self.text
        );
    }

    pub fn show_warning(&self) {
        println!(
            "- [{}]: ({}:{}) -> {}",
            "warning".yellow(),
            self.line + 1,
            self.column + 1,
            self.text
        );
    }
}
