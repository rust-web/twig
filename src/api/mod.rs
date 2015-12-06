/*!
Twig extension writer's API.
*/

pub mod tokens;
pub mod lexer;

/// Line-column position in a file.
#[derive(Debug, Default, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
