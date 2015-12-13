// This file is part of rust-web/twig
//
// For the copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Lexes a template string.

use template;
use api::token;
use api::error::Traced;

pub mod job;
pub mod error;
pub mod options;
pub use self::error::{LexerError, SyntaxError};
pub use self::options::Options;


#[derive(PartialEq, Debug, Default)]
pub struct Lexer;

impl Lexer {
    pub fn new(_opt: Options) -> Result<Lexer, Traced<LexerError>> {
        unimplemented!()
    }

    #[allow(dead_code)] // TODO: testcase
    pub fn tokenize<'a, 't> (&'a self, _template: &'t template::Raw) -> Result<token::Stream<'t>, Traced<LexerError>>
        where 't: 'a // the template must outlive the Lexer
    {
        unimplemented!()
    }
}
