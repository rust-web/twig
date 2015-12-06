use std::io;
use api::error::SyntaxError;

#[derive(Debug)]
pub enum LexingError {
    Syntax(SyntaxError),
    Io(io::Error)
}

impl From<io::Error> for LexingError {
    fn from(other: io::Error) -> LexingError {
        LexingError::Io(other)
    }
}

impl From<SyntaxError> for LexingError {
    fn from(other: SyntaxError) -> LexingError {
        LexingError::Syntax(other)
    }
}
