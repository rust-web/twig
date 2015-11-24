// This file is part of Twig (ported to Rust).
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

/// Rust macro
///
/// @author Colin Kiegel <kiegel@gmx.de>


/////////////
// exports //
/////////////

pub use super::*;

#[macro_export]
macro_rules! err {
    ( $code:expr ) => ({
        ::error::Exception::new($code, err_location!())
    });
}

#[macro_export]
macro_rules! err_location {
    () => ({
        ::error::Location {
            module_path : module_path!(),
            filename : file!(),
            line : line!(),
            column : column!(),
        }
    });
}

// NOTE: because convert::From<T> already is reflexive (generic `impl From<T> for T`)
//       we can't generically `impl From<Exception<A>> for Exception<B> where ...`
//       - too bad!
//       TODO: file a bug!

#[macro_export]
macro_rules! impl_convert_exception {
    ( $source_type:ty, $target_type:ty, $target_error_code:expr ) => (
        impl ::std::convert::From<::error::Exception<$source_type>> for ::error::Exception<$target_type> {
            fn from(cause: ::error::Exception<$source_type>) -> ::error::Exception<$target_type> {

                // NOTE can't use `err_location!(None)` here
                //      because that would yield linenumbers from
                //      calls to impl_convert_exception and not from
                //      the implicit call-site of .into() ...
                //
                //      alternative: manual conversion via different macro
                //          try_into!() or sth. like that?

                ::error::Exception::new($target_error_code, cause.location())
                    .caused_by(cause)
            }
        }
    );
}


// `try_chain!`-macro will create an error-chain with location for each chaining-operation
//
// use as follows
//
// fn foo() -> Result<(), Exception<CODE_A>> {
//    let result_B: Result<(), Exception<CODE_B>> = ...;
//
//    try_chain!(result_B);
// }
macro_rules! try_chain {
    ( $result:expr ) => (
        match($result) {
            Ok(value) => value,
            Err(cause) => {
                return err!(::error::api::GeneralizeTo::generalize(cause.code()))
                    .caused_by(cause)
                    .into()
            }
        }
    )
}
