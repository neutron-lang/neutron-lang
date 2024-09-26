/// A struct that stores a position -> (line and column)
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
