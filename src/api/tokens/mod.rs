// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/*!
Tokens, received from lexer output.
*/

/// Token value.
///
/// The lifetime of this token refers to original source string which
/// should be kept alive as long as this token.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenRef<'a> {
    BlockStart,
    Name(&'a str),
    Text(&'a str),
}

impl<'a> From<TokenRef<'a>> for Token {
    /// Get owned value for this token.
    fn from<'r>(other: TokenRef<'r>) -> Self {
        match other {
            TokenRef::BlockStart => Token::BlockStart,
            TokenRef::Name(v) => Token::Name(v.into()),
            TokenRef::Text(v) => Token::Text(v.into()),
        }
    }
}

/// Owned token value.
#[derive(Debug, Clone)]
pub enum Token {
    BlockStart,
    Name(String),
    Text(String),
}
