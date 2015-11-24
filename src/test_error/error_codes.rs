// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig library for rust

#[macro_use]
use error::Exception;
use error::api::GeneralizeTo;
use std::fmt::{self, Display};


pub type EngineError = Exception<EngineErrorCode>;
pub type EngineResult<T> = Result<T, EngineError>;

pub type RuntimeError = Exception<RuntimeErrorCode>;
pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl GeneralizeTo<EngineErrorCode> for RuntimeErrorCode {
    fn generalize(&self) -> EngineErrorCode {
        EngineErrorCode::RuntimeError
    }
}

#[derive(Clone, Debug)]
pub enum EngineErrorCode {
    TemplateNotFound { name: String, search_paths: Vec<String> },
    RuntimeError
}

impl Display for EngineErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EngineErrorCode::TemplateNotFound { ref name, ref search_paths } => {
                if search_paths.len() == 0 {
                    // note: use {:?} instead of \"{}\" for strings
                    //      -> easier to write
                    //      -> escapes quotes within strings
                    write!(f, "Template {:?} was not found", name)
                } else {
                    try!(write!(f, "Template {:?} was not found, looked in ", name));
                    write!(f, "{:?}", search_paths)
                }
            },
            EngineErrorCode::RuntimeError => {
                write!(f, "Some Runtime Error")
            }
        }
    }
}

#[derive(Clone, Debug)]
/// Runtime error message.
pub enum RuntimeErrorCode {
    /// Callable invoked with argument count that does not match defined count.
    InvalidArgumentCount { defined: usize, given: usize },
}

impl fmt::Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeErrorCode::InvalidArgumentCount { ref defined, ref given } => {
                write!(f, "Target requires {} arguments, called with {}", defined, given)
            }
        }
    }
}
