// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::fmt::{self, Display};
use std::error::Error;

use loader::LoaderError;
use api::parser::ParserError;
use api::lexer::LexerError;
use api::ext;
use std::convert::From;


#[derive(Debug)]
pub enum TwigError {
    Loader(LoaderError),
    LoaderNotInitialized,
    Lexer(LexerError),
    LexerNotInitialized,
    Parser(ParserError),
    Runtime,
    ExtensionRegistry(ExtensionRegistryError),
}

impl From<LoaderError> for TwigError {
    fn from(err: LoaderError) -> TwigError {
        TwigError::Loader(err)
    }
}

impl From<LexerError> for TwigError {
    fn from(err: LexerError) -> TwigError {
        TwigError::Lexer(err)
    }
}

impl From<ParserError> for TwigError {
    fn from(err: ParserError) -> TwigError {
        TwigError::Parser(err)
    }
}

impl From<ExtensionRegistryError> for TwigError {
    fn from(err: ExtensionRegistryError) -> TwigError {
        TwigError::ExtensionRegistry(err)
    }
}

impl Error for TwigError {
    fn description(&self) -> &str {
        match *self {
            TwigError::Loader(..) => "Twig loader error.",
            TwigError::LoaderNotInitialized => "The template loader must be initializied prior usage.",
            TwigError::Lexer(..) => "Twig lexer error.",
            TwigError::LexerNotInitialized => "The template lexer must be initializied prior usage.",
            TwigError::Parser(..) => "Twig parser error.",
            TwigError::Runtime => "Twig runtime error.",
            TwigError::ExtensionRegistry(..) => "Twig extension registry error."
        }
    }
}

impl Display for TwigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            TwigError::Loader(ref e) => e.fmt(f),
            TwigError::Lexer(ref e) => e.fmt(f),
            TwigError::Parser(ref e) => e.fmt(f),
            TwigError::ExtensionRegistry(ref e) => e.fmt(f),
            TwigError::LoaderNotInitialized
            | TwigError::LexerNotInitialized
            | TwigError::Runtime
            => Ok(())
        }
    }
}

#[derive(Debug)]
pub enum ExtensionRegistryError {
    /// To be used by custom implementations of `twig::api::ext::Extension::init()`
    ExtensionInitFailure {
        reason: String,
    },
    DuplicateExtension {
        name: String,
    },
    DuplicateFilter {
        prev: Box<ext::Filter>
    },
    DuplicateFunction {
        prev: Box<ext::Function>
    },
    DuplicateOperatorUnary {
        prev: ext::UnaryOperator
    },
    DuplicateOperatorBinary {
        prev: ext::BinaryOperator
    },
    DuplicateTest {
        prev: Box<ext::Test>
    },
    DuplicateTagHandler {
        prev: Box<ext::TokenParser>
    },
    DuplicateTokenParser {
        prev: Box<ext::TokenParser>
    },
}

impl Error for ExtensionRegistryError {
    fn description(&self) -> &str {
        match *self {
            ExtensionRegistryError::ExtensionInitFailure{..} => "Engine extension failed to initialize.",
            ExtensionRegistryError::DuplicateExtension{..} => "Duplicate extension.",
            ExtensionRegistryError::DuplicateFilter{..} => "Duplicate filter.",
            ExtensionRegistryError::DuplicateFunction{..} => "Duplicate function.",
            ExtensionRegistryError::DuplicateOperatorBinary{..} => "Duplicate binary operator.",
            ExtensionRegistryError::DuplicateOperatorUnary{..} => "Duplicate unary operator.",
            ExtensionRegistryError::DuplicateTest{..} => "Duplicate test.",
            ExtensionRegistryError::DuplicateTagHandler{..} => "Duplicate tag handler.",
            ExtensionRegistryError::DuplicateTokenParser{..} => "Duplicate token parser.",
        }
    }
}

impl Display for ExtensionRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            ExtensionRegistryError::ExtensionInitFailure {
                ref reason
            } => {
                write!(f, " {}", reason)
            },
            ExtensionRegistryError::DuplicateExtension {
                ref name
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = name)
            },
            ExtensionRegistryError::DuplicateFilter {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryError::DuplicateFunction {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryError::DuplicateOperatorBinary {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryError::DuplicateOperatorUnary {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryError::DuplicateTest {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryError::DuplicateTagHandler {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryError::DuplicateTokenParser {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            }
        }
    }
}
