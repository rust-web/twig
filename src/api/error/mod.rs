mod syntax;
mod lexing;

use std::result;

pub use self::lexing::LexingError;
pub use self::syntax::{ SyntaxError, SyntaxErrorCode, UnexpectedToken };

pub type LexingResult<T> = result::Result<T, LexingError>;
pub type ParsingResult<T> = result::Result<T, SyntaxError>;
