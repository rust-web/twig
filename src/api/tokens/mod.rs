/*!
Tokens, received from lexer output.
*/

/// Token value.
///
/// The lifetime of this token refers to original source string which
/// should be kept alive as long as this token.
#[derive(Debug)]
pub enum TokenRef<'a> {
    Text(&'a str),
}

impl<'a> TokenRef<'a> {
    /// Get owned value for this token.
    pub fn into_token(self) -> Token {
        match self {
            TokenRef::Text(v) => Token::Text(v.into()),
        }
    }
}

/// Owned token value.
pub enum Token {
    Text(String),
}
