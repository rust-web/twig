// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust

use error::{Error, ErrorCode, GeneralizeTo};
use std::fmt::{self, Display};


pub type EngineError = Error<EngineErrorCode>;
pub type EngineResult<T> = Result<T, EngineError>;

pub type RuntimeError = Error<RuntimeErrorCode>;
pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl GeneralizeTo<EngineErrorCode> for RuntimeErrorCode {
    fn generalize(&self) -> EngineErrorCode {
        EngineErrorCode::RuntimeError
    }
}

#[derive(Debug)]
pub enum EngineErrorCode {
    TemplateNotFound { name: String, search_paths: Vec<String> },
    RuntimeError
}

impl ErrorCode for EngineErrorCode {
    fn description(&self) -> &str {
        match *self {
            EngineErrorCode::TemplateNotFound{..} => "Template not found",
            EngineErrorCode::RuntimeError => "Runtime error",
        }
    }
}

impl Display for EngineErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EngineErrorCode::TemplateNotFound { ref name, ref search_paths } => {
                // note: use {:?} instead of \"{}\" for strings
                //      -> easier to write
                //      -> escapes quotes within strings
                write!(f, "Template {:?} not found, looked in {:?}", name, search_paths)
            },
            EngineErrorCode::RuntimeError => {
                write!(f, "Some runtime error")
            }
        }
    }
}

// Just for demonstration
// - lexer and parser could define some objects to indicate their current position
// .. however we call that
#[derive(Debug)]
pub struct MockCursor;

impl Display for MockCursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{template_name}` line {line} column {column}",
            template_name = "index.twig.html",
            line = 11,
            column = 2)
    }
}
//

#[derive(Debug)]
/// Runtime error message.
pub enum RuntimeErrorCode {
    /// Callable invoked with argument count that does not match defined count.
    InvalidArgumentCount { defined: usize, given: usize, cursor: MockCursor },
}

impl ErrorCode for RuntimeErrorCode {
    fn description(&self) -> &str {
        match *self {
            RuntimeErrorCode::InvalidArgumentCount{..} => "Invalid Argument Count",
        }
    }
}

impl Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeErrorCode::InvalidArgumentCount { ref defined, ref given, ref cursor } => {
                write!(f, "Target requires {defined} arguments, called with {given} for {cursor}",
                    defined = defined,
                    given = given,
                    cursor = cursor)
            }
        }
    }
}