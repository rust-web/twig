// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Twig base exception

use std::fmt::{self, Debug, Display};
use std::any::Any;
use std::convert::Into;


#[macro_use]
pub mod macros;
// use std Error-trait to improve cross-crate compatibility
// don't mix it up with Err(X)
pub use std::error::Error;
pub mod api;

// however this means, we have to call our objects differently ... I suggest Exception
pub struct Exception<T>
    where T: Any + Debug + Display
{
    // the exception codes are going to be enums
    // - i.e. Exception<MY_ENUM> implements std::error::Error without any boilerplate
    // to MY_ENUM. Hurray! :-)
    code: T,
    // I decided to call this field `code` instead of `error` to not confuse it with the Error trait
    location: Location,
    // chaining is required by std::error::Error
    cause: Option<Box<Error>>,
    // description buffer is required by std::error::Error - it is a compilation of code + location
    //
    // It's hard to compile this string lazily - even with Option<RefCell<>>, due to the API
    // of std::error::Error. So we have to precompile this string for now, even if someone
    // just throws away the Exception. :-/
    description: String
}

impl<T> Exception<T>
    where T: Any + Debug + Display
{
    pub fn new(code: T, location: Location) -> Exception<T> {
        let description = format!("{error_code} at {location}",
            error_code = code,
            location = location.to_string());

        Exception {
            code: code,
            location: location,
            cause: None,
            description: description
        }
    }

    #[allow(dead_code)] // only used by tests
    pub fn code(&self) -> &T {
        &self.code
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    // I would opt to remove(!) this method in favour of more complex enum error-codes
    // with additional data like you are using right now. Removal would force us to put
    // all useful information into the enums. But that might turn out to be useful afterwards.
    // We could gain much information by just Debug-formatting our errors / exceptions.
    // Good-Bye pre-parsed strings. ;-)
    pub fn explain(mut self, _message: String) -> Self {
        unimplemented!()
    }

    pub fn caused_by<X: 'static + Error>(mut self, cause: X) -> Self {
        self.cause = Some(Box::new(cause));

        self
    }

    pub fn causes<X>(self, wrapper: Exception<X>) -> Exception<X> where
        X: Any + Debug + Display
    {
        wrapper.caused_by(self)
    }

    // iterate along the error-chain.
    pub fn iter(&self) -> ErrorIter {
        ErrorIter {
            next: Some(self),
        }
    }
}

impl<T> Error for Exception<T>
    where T: Any + Debug + Display
{
    fn description(&self) -> &str {
        &self.description
    }

    fn cause<'a>(&'a self) -> Option<&'a Error> {
        self.cause.as_ref().map(|x| &**x) // dereference from Option<Box<T>> to Option<&T>
    }
}

// Exception -> Err(Exception)
impl<T, V> Into<Result<V, Exception<T>>> for Exception<T>
    where T: Any + Debug + Display
{
    fn into (self) -> Result<V, Exception<T>> {
        Err(self)
    }
}



pub struct ErrorIter<'a> {
    next: Option<&'a Error>
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = &'a Error;

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

impl<T> Display for Exception<T>
    where T: Any + Debug + Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let recursive_desc = self.iter().map(|e| e.description())
             .collect::<Vec<&str>>().join(" caused by\n - ");
        write!(f, "\n - {}\n", recursive_desc)
    }
}

impl<T> Debug for Exception<T>
    where T: Any + Debug + Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // custom implementation - skipping description, which would be somewhat redundant
        f.debug_struct("Exception<T>")
            .field("location", &self.location)
            .field("code", &self.code)
            .field("cause", &self.cause)
            .finish()
    }
}

#[derive(Debug)]
pub struct Location {
    // this might be a bit redundant - but we just store everything we can get.
    // we don't need to be super performant on exceptions - because we try to avoid them :-)
    //
    // note that the module_path is currently only displayed in Debug output due to this redundancy
    pub module_path : &'static str, // e.g. twig::lexer::job::state::shared_traits
    pub filename : &'static str,    // e.g. /src/lexer/job/state/shared_traits.rs
    pub line : u32,
    pub column : u32,
}

impl ToString for Location {
    fn to_string(&self) -> String {
        format!("{filename}:{line}:{column}",
            filename = self.filename,
            line     = self.line,
            column   = self.column)
    }
}
