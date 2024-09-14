use colored::Colorize;

pub struct Message {
    pub file: String,
    pub text: String,
    pub line: usize,
    pub column: usize
}

impl Message {
    pub fn show_error(&self) {
        println!("[{}: {},{}] -> {}: {}", self.file, self.line, self.column, "error".red(), self.text)
    }
    
    pub fn show_warning(&self) {
        println!("[{}: {},{}] -> {}: {}", self.file, self.line, self.column, "warning".yellow(), self.text)
    }
}