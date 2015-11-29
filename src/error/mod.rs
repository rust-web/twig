// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig Error Handling

use std::fmt::{self, Debug, Display};
use std::error;
use std::any::Any;

#[macro_use]
mod macros;
// use std Error-trait to improve cross-crate compatibility
// don't mix it up with Err(X)


/// Lightweight base functionality for error codes
///
/// Similar to std::error::Error, but _without_ error-chaining.
pub trait ErrorCode: Debug + Display + Any {
    /// A short description of the error code.
    ///
    /// The description should not contain newlines or sentence-ending
    /// punctuation, to facilitate embedding in larger user-facing
    /// strings.
    fn description(&self) -> &str;

    /// Returns generic twig error for this error code.
    /// You must provide the location, where the error occured.
    fn at(self, location: Location) -> Error<Self> where
        Self: Sized
    {
        Error::new(self, location)
    }
}

/// Reference implementation to make examples easier.
impl ErrorCode for &'static str {
    fn description(&self) -> &str { *self }
}

/// Transform ErrorCodes
///
/// It is different to the Into-trait, because we only take references.
pub trait GeneralizeTo<T> {
    fn generalize(&self) -> T;
}

/// Record current state of complex objects
///
/// Implement this trait for complex objects. Make sure, that the Dump::Data type
/// does not contain lifetimes to keep error codes simple. In practice this means
/// cloning all referenced data into the dump.
///
/// For any `X: Dump` you can
///
/// * reference the associated type via `<X as Dump>::Data`
/// * create the dump via `X.dump()`
pub trait Dump {
    type Data: Debug + Display + 'static;

    fn dump(&self) -> Self::Data;
}

/// Generic twig error
///
/// Wrapper around some `ErrorCode` - adds location support and error-chaining.
#[derive(Debug)]
pub struct Error<T>
    where T: ErrorCode
{
    // the exception codes are going to be enums
    // - i.e. Exception<MY_ENUM> implements std::error::Error without any boilerplate
    // to MY_ENUM. Hurray! :-)
    code: T,
    // I decided to call this field `code` instead of `error` to not confuse it with the Error trait
    location: Location,
    // chaining is required by std::error::Error
    cause: Option<Box<error::Error>>,
}

impl<T> Error<T>
    where T: ErrorCode
{
    /// Create a new twig error out of some generic error code.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate twig;
    /// # fn main() {
    /// Error::new("my error", loc!()); // shorthand: `err!("my error")`
    /// # }
    /// ```
    pub fn new(code: T, location: Location) -> Error<T> {
        Error {
            code: code,
            location: location,
            cause: None
        }
    }

    /// Return the associated error code.
    #[allow(dead_code)] // only used by tests
    pub fn code(&self) -> &T {
        &self.code
    }

    /// Return the location the error occured.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Set the cause for this error.
    pub fn caused_by<X: 'static + error::Error>(mut self, cause: X) -> Self {
        self.cause = Some(Box::new(cause));

        self
    }

    /// Wraps this error inside another error as its cause.
    pub fn causes<X>(self, wrapper: Error<X>) -> Error<X> where
        X: ErrorCode
    {
        wrapper.caused_by(self)
    }

    /// Creates an iterator to iterate along the error cause-chain.
    pub fn iter(&self) -> ErrorIter {
        ErrorIter {
            next: Some(self),
        }
    }
}

impl<T> error::Error for Error<T>
    where T: ErrorCode
{
    fn description(&self) -> &str {
        // delegate the error description to the ErrorCode
        &self.code.description()
    }

    fn cause<'a>(&'a self) -> Option<&'a error::Error> {
        // dereference from Option<Box<T>> to Option<&T>
        self.cause.as_ref().map(|x| &**x)
    }
}

/// Iterator to iterate along the error cause-chain.
pub struct ErrorIter<'a> {
    next: Option<&'a error::Error>
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = &'a error::Error;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.next {
            Some(err) => {
                self.next = err.cause();
                Some(err)
            }
            None => None,
        }
    }
}

impl<T> Display for Error<T>
    where T: ErrorCode
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "{error_code} at {location}\n",
            error_code = self.code,
            location = self.location));

        match self.cause {
            None => Ok(()),
            Some(ref cause) => write!(f, " - caused by: {}", cause),
        }
    }
}

/// Location in rust source code
///
/// The Display trait is formatted like `"{filename}:{line}:{column}"`.
///
/// We just store everything we can get, to identify source code locations. Note that
/// the module_path is currently only displayed in Debug output due to this redundancy.
/// You can access all fields directly.
#[derive(Debug)]
pub struct Location {
    pub module_path : &'static str, // e.g. twig::lexer::job::state::shared_traits
    pub filename : &'static str,    // e.g. /src/lexer/job/state/shared_traits.rs
    pub line : u32,
    pub column : u32,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{filename}:{line}:{column}",
            filename = self.filename,
            line     = self.line,
            column   = self.column)
    }
}