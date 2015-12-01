// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Represents a token stream.

use std::fmt;
use engine::parser::token::{self, Token};
use template;
use engine::parser::token::{TokenError, TokenErrorCode};
use error::Dump;

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{line}:{column}",
            line = self.line,
            column = self.column)
    }
}

#[derive(Debug)]
pub struct Item {
    token: Token,
    position: Position,
}

impl Item {
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn expect<T>(&self, pattern: T, reason: Option<&'static str>) -> Result<&Item, TokenError>
        where T: token::Pattern + 'static
    {
        if pattern.matches(self.token()) {
            Ok(&self)
        } else {
            err!(TokenErrorCode::UnexpectedTokenAtItem {
                reason: reason,
                expected: <token::Pattern as Dump>::dump(&pattern),
                found: self.dump(),
            })
        }
    }
}

pub type ItemDump = Item; // may change as soon as we use RefTokens

impl Dump for Item {
    type Data = ItemDump;

    fn dump(&self) -> Self::Data {
        ItemDump {
            token: self.token.dump(),
            position: self.position.clone()
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "token {t} at position {p}",
            t = self.token,
            p = self.position)
    }
}

//#[derive(Default)]
pub struct Stream<'a> {
    items: Vec<Item>,
    _template: &'a template::Raw,
}

impl<'a> fmt::Display for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let v: Vec<String> = self.items.iter().map(|i| format!("{}", i.token)).collect();
        write!(f, "[\n\t{}\n]", v.join("\n\t"))
    }
}

impl<'a> fmt::Debug for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let v: Vec<String> = self.items.iter().map(|i| format!("{:?}", i.token)).collect();
        write!(f, "[\n\t{}\n]", v.join("\n\t"))
    }
}

#[derive(Debug)]
pub struct StreamDump {
    pub template_str: String,
    pub items_str: String,
}

impl<'a> Dump for Stream<'a> {
    type Data = StreamDump;

    fn dump(&self) -> Self::Data {
        unimplemented!()
    }
}

impl fmt::Display for StreamDump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " StreamDump{{ template: {template:?}, items: {items:?}}}",
            template = self.template_str,
            items = self.items_str)
    }
}

impl<'a> IntoIterator for Stream<'a> {
    type Item = self::Item;
    type IntoIter = <Vec<self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
