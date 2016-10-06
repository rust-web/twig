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

quick_error! {
    #[derive(Debug)]
    pub enum TwigError {
        Loader(cause: LoaderError) {
            description("Twig loader error")
            display(me) -> ("{}: {}", me.description(), cause)
            from()
            cause(&*cause)
        }
        LoaderNotInitialized {
            description("The template loader must be initializied prior usage.")
        }
        Lexer(cause: LexerError) {
            description("Twig lexer error")
            display(me) -> ("{}: {}", me.description(), cause)
            from()
            cause(&*cause)
        }
        LexerNotInitialized {
            description("The template lexer must be initializied prior usage.")
        }
        Parser(cause: ParserError) {
            description("Twig parser error")
            display(me) -> ("{}: {}", me.description(), cause)
            from()
            cause(&*cause)
        }
        Runtime {
            description("Twig runtime error.")
        }
        ExtensionRegistry(cause: ExtensionRegistryError) {
            description("Twig extension registry error")
            display(me) -> ("{}: {}", me.description(), cause)
            from()
            cause(&*cause)
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
