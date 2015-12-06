// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::io;
use api::error::SyntaxError;
use std::result;

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

pub type LexingResult<T> = result::Result<T, LexingError>;
