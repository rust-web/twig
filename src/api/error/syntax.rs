// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use api::Position;
use api::tokens::{ Token };

#[derive(Debug)]
pub struct SyntaxError {
    pub code: SyntaxErrorCode,
    pub starts_at: Position,
    pub ends_at: Option<Position>,
}

impl SyntaxError {
    /// Specify the location where the error ends.
    pub fn ends_at(mut self, pos: Position) -> SyntaxError {
        self.ends_at = Some(pos);
        self
    }
}

#[derive(Debug)]
pub enum SyntaxErrorCode {
    ExpectedTokenButReceived { expected: Token, received: UnexpectedToken },
}

impl SyntaxErrorCode {
    /// Specify the location where the error starts.
    pub fn starts_at(self, pos: Position) -> SyntaxError {
        SyntaxError {
            code: self,
            starts_at: pos,
            ends_at: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnexpectedToken {
    Token(Token),
    EndOfStream,
}
