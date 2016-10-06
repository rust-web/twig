// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Typisation of syntax errors.

use std::error::Error;
use std::convert::From;

pub type LoaderError = Box<Error>; // unimplemented!()
pub type ParserError = Box<Error>; // unimplemented!()
pub type LexerError = Box<Error>; // unimplemented!()
pub type ExtensionRegistryError = Box<Error>; // unimplemented!()

quick_error! {
    #[derive(Debug)]
    pub enum TwigError {
        Loader(cause: LoaderError) {
            description("Twig loader error")
            display(me) -> ("{}: {}", me.description(), cause)
            from()
            cause(cause.as_ref())
        }
        Lexer(cause: LexerError) {
            description("Twig lexer error")
            display(me) -> ("{}: {}", me.description(), cause)
            //from()
            cause(cause.as_ref())
        }
        Parser(cause: ParserError) {
            description("Twig parser error")
            display(me) -> ("{}: {}", me.description(), cause)
            //from()
            cause(cause.as_ref())
        }
        ExtensionRegistry(cause: ExtensionRegistryError) {
            description("Twig extension registry error")
            display(me) -> ("{}: {}", me.description(), cause)
            //from()
            cause(cause.as_ref())
        }
    }
}
