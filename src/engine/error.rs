// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::fmt::{self, Display};
use error::Error;
use error::{GeneralizeTo, ErrorCode};

use loader::LoaderErrorCode;
use api::parser::ParserErrorCode;
use api::lexer::LexerErrorCode;
use api::ext;

pub type TwigError = Error<TwigErrorCode>;
pub type ExtensionRegistryError = Error<ExtensionRegistryErrorCode>;

#[derive(Debug)]
pub enum TwigErrorCode {
    Loader,
    LoaderNotInitialized,
    Lexer,
    LexerNotInitialized,
    Parser,
    Runtime,
    ExtensionRegistry,
}

impl GeneralizeTo<TwigErrorCode> for LoaderErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::Loader }
}

impl GeneralizeTo<TwigErrorCode> for LexerErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::Lexer }
}

impl GeneralizeTo<TwigErrorCode> for ParserErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::Parser }
}

impl GeneralizeTo<TwigErrorCode> for ExtensionRegistryErrorCode {
    fn generalize(&self) -> TwigErrorCode { TwigErrorCode::ExtensionRegistry }
}

impl ErrorCode for TwigErrorCode {
    fn description(&self) -> &str {
        match *self {
            TwigErrorCode::Loader => "Twig loader error.",
            TwigErrorCode::LoaderNotInitialized => "The template loader must be initializied prior usage.",
            TwigErrorCode::Lexer => "Twig lexer error.",
            TwigErrorCode::LexerNotInitialized => "The template lexer must be initializied prior usage.",
            TwigErrorCode::Parser => "Twig parser error.",
            TwigErrorCode::Runtime => "Twig runtime error.",
            TwigErrorCode::ExtensionRegistry => "Twig extension registry error."
        }
    }
}

impl Display for TwigErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            TwigErrorCode::Loader
            | TwigErrorCode::LoaderNotInitialized
            | TwigErrorCode::Lexer
            | TwigErrorCode::LexerNotInitialized
            | TwigErrorCode::Parser
            | TwigErrorCode::Runtime
            | TwigErrorCode::ExtensionRegistry
            => Ok(())
        }
    }
}

#[derive(Debug)]
pub enum ExtensionRegistryErrorCode {
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

impl ErrorCode for ExtensionRegistryErrorCode {
    fn description(&self) -> &str {
        match *self {
            ExtensionRegistryErrorCode::ExtensionInitFailure{..} => "Engine extension failed to initialize.",
            ExtensionRegistryErrorCode::DuplicateExtension{..} => "Duplicate extension.",
            ExtensionRegistryErrorCode::DuplicateFilter{..} => "Duplicate filter.",
            ExtensionRegistryErrorCode::DuplicateFunction{..} => "Duplicate function.",
            ExtensionRegistryErrorCode::DuplicateOperatorBinary{..} => "Duplicate binary operator.",
            ExtensionRegistryErrorCode::DuplicateOperatorUnary{..} => "Duplicate unary operator.",
            ExtensionRegistryErrorCode::DuplicateTest{..} => "Duplicate test.",
            ExtensionRegistryErrorCode::DuplicateTagHandler{..} => "Duplicate tag handler.",
            ExtensionRegistryErrorCode::DuplicateTokenParser{..} => "Duplicate token parser.",
        }
    }
}

impl Display for ExtensionRegistryErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.description()));

        match *self {
            ExtensionRegistryErrorCode::ExtensionInitFailure {
                ref reason
            } => {
                write!(f, " {}", reason)
            },
            ExtensionRegistryErrorCode::DuplicateExtension {
                ref name
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = name)
            },
            ExtensionRegistryErrorCode::DuplicateFilter {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryErrorCode::DuplicateFunction {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryErrorCode::DuplicateOperatorBinary {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryErrorCode::DuplicateOperatorUnary {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryErrorCode::DuplicateTest {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryErrorCode::DuplicateTagHandler {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            },
            ExtensionRegistryErrorCode::DuplicateTokenParser {
                prev: ref p
            } => {
                write!(f, " {prev:?} has already been registered.",
                    prev = p)
            }
        }
    }
}
