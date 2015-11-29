// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Twig macros for error handling

/// A macro which creates a error for the location from which it was invoked.
/// For internal use within the twig library.
///
/// The expanded expression has type `Result<_,twig::error::Error<T>>`, where the suplied
/// error code must implement `twig::error::api::ErrorCode`.
///
/// # Examples
///
/// ```rust,macro_test
/// # #[macro_use] extern crate twig;
/// # fn main() {
/// use twig::error::Error;
///
/// let result: Result<(), Error<&'static str>> = err!("critical error");
/// if let Err(error) = result {
///     assert_eq!(error.to_string(), "critical error at <anon>:5:46\n");
/// }
/// # }
/// ```
#[macro_export]
macro_rules! err {
    ( $code:expr ) => ({
        Err($crate::error::Error::new($code, loc!()))
    });
}
/// A macro which expands to the location from which it was invoked.
/// For internal use within the twig library.
///
/// The expanded expression has type `twig::error::Location`, and the returned location
/// is not the invocation of the `loc!()` macro itself, but rather the
/// first macro invocation leading up to the invocation of the `loc!()`
/// macro.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate twig;
/// # fn main() {
/// use twig::error;
///
/// let this_location = loc!();
/// println!("called from: {}", this_location);
/// # }
/// ```
#[macro_export]
macro_rules! loc {
    () => ({
        $crate::error::Location {
            module_path : module_path!(),
            filename : file!(),
            line : line!(),
            column : column!(),
        }
    });
}

/// A macro which will create an error-chain with location for each chaining-operation.
/// For internal use within the twig library.
///
/// `try_chain!` is supposed to be used, whenever errors cross a logic boundary. The trait
/// `twig::error::api::GeneralizeTo<CODE_A>` must be implented for `CODE_B`, then use it as follows (pseudo-code)
///
/// ```ignore
/// fn foo() -> Result<(), Exception<CODE_A>> {
///    let result_B: Result<(), Exception<CODE_B>> = ...;
///
///    try_chain!(result_B); // try! would fail here, and
/// }
/// ```
#[macro_export]
macro_rules! try_chain {
    ( $result:expr ) => (
        match $result {
            Ok(value) => value,
            Err(cause) => {
                let code = $crate::error::GeneralizeTo::generalize(cause.code());

                return Err($crate::error::Error::new(code, loc!())
                    .caused_by(cause)
                    .into())
            }
        }
    )
}
