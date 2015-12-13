// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::error::Error;
use std::fmt::{self, Display};
use std::convert::From;

pub type LoaderError = Box<Error>; // unimplemented!()
pub type ParserError = Box<Error>; // unimplemented!()
pub type LexerError = Box<Error>; // unimplemented!()
pub type ExtensionRegistryError = Box<Error>; // unimplemented!()


#[derive(Debug)]
pub enum TwigError {
    Loader(LoaderError),
    Lexer(LexerError),
    Parser(ParserError),
    ExtensionRegistry(ExtensionRegistryError),
}

impl From<LoaderError> for TwigError {
    fn from(err: LoaderError) -> TwigError {
        TwigError::Loader(err)
    }
}

// Due to the `dummy` error impls we can not define different From-impls
//
// impl From<LexerError> for TwigError {
//     fn from(err: LexerError) -> TwigError {
//         TwigError::Lexer(err)
//     }
// }
//
// impl From<ParserError> for TwigError {
//     fn from(err: ParserError) -> TwigError {
//         TwigError::Parser(err)
//     }
// }
//
// impl From<ExtensionRegistryError> for TwigError {
//     fn from(err: ExtensionRegistryError) -> TwigError {
//         TwigError::ExtensionRegistry(err)
//     }
// }

impl Error for TwigError {
    fn description(&self) -> &str {
        match *self {
            TwigError::Loader(..) => "Twig loader error.",
            TwigError::Lexer(..) => "Twig lexer error.",
            TwigError::Parser(..) => "Twig parser error.",
            TwigError::ExtensionRegistry(..) => "Twig extension registry error."
        }
    }
}

impl Display for TwigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            TwigError::Loader(ref e) => Display::fmt(e,f),
            TwigError::Lexer(ref e) => Display::fmt(e,f),
            TwigError::Parser(ref e) => Display::fmt(e,f),
            TwigError::ExtensionRegistry(ref e) => Display::fmt(e,f),
        }
    }
}
