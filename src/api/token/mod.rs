// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Represents a Token

use std::fmt;
use error::Dump;

pub mod error;
pub mod stream;
pub use self::stream::Stream;
pub use self::error::TokenError;
pub use self::error::TokenErrorCode;

#[derive(PartialEq, Clone)]
pub enum Token {
    _Eof,
    Text(String),
    // ..
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuation {
    Dot,
    Comma,
    Colon,
    VerticalBar,
    QuestionMark,
    OpeningBracket(BracketType),
    ClosingBracket(BracketType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BracketType {
    Round,
    Square,
    Curly,
    DoubleQuote, // Pseudo-Bracket - never being pushed to a real token Stream
                 // but used as a temporary state of the lexer
}

#[derive(PartialEq)]
/// Used to define patterns about expected tokens.
///
/// Example: `token::Type::Text`
pub enum Type {
    Eof,
    Text,
    // ..
}

#[allow(unused_variables)]
impl Token {
    // TODO store String representation for numbers and Punctuation?
    // NOTE: Because of Number Types we need to return `String` copys instead of `&'a str`
    pub fn value<'a>(&'a self) -> Option<String> {
        match *self {
            Token::_Eof => None,
            Token::Text(ref x) => Some(x.to_string()),
            // ..
        }
    }

    // NOTE: Does *not* yield number types - use value() instead.
    pub fn value_as_str<'a>(&'a self) -> Option<&str> {
        match *self {
            Token::_Eof => None,
            Token::Text(ref x) => Some(x),
            // ..
        }
    }

    pub fn get_type(&self) -> Type {
        match *self {
            Token::_Eof => Type::Eof,
            Token::Text(_) => Type::Text,
            // ..
        }
    }

    #[allow(dead_code)] // TODO: testcase
    pub fn is_type(&self, typ: Type) -> bool {
        self.get_type() == typ
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.get_type().name())
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let typ = self.get_type().name();
        match self.value() {
            Some(ref val) => write!(f, "{typ}({val:?})", typ = typ, val = val),
            None          => write!(f, "{typ}", typ = typ),
        }
    }
}

pub type TokenDump = Token; // may change as soon as we use RefTokens

impl Dump for Token {
    type Data = TokenDump;

    fn dump(&self) -> Self::Data {
        self.clone()
    }
}

#[allow(unused_variables)]
impl Type {
    /// Returns the name of the token type (internal representation).
    pub fn name(&self) -> &'static str {
         match *self {
            Type::Eof => "EOF",
            Type::Text => "TEXT",
            // ..
        }
    }

    /// Returns the description of the token type in plain english.
    pub fn _description(&self) -> &'static str {
         match *self {
            Type::Eof => "end of template",
            Type::Text => "text",
            // ..
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

pub trait Pattern: fmt::Debug + fmt::Display {
    fn matches(&self, &Token) -> bool;
}

impl Pattern for Token {
    fn matches(&self, token: &Token) -> bool {
        *self == *token
    }
}

impl Pattern for Type {
    fn matches(&self, token: &Token) -> bool {
        *self == token.get_type()
    }
}

pub type PatternDump = String;

impl Dump for Pattern {
    type Data = PatternDump;

    fn dump(&self) -> Self::Data {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Type};

    #[test]
    fn new_token() {
        let token = Token::Text("Hello World!".to_string());
        assert_eq!(token.value().unwrap(), "Hello World!".to_string());
        assert!(token.is_type(Type::Text));
    }
}
