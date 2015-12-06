use api::Position;
use api::tokens::TokenRef;

use super::{ Lexer };

/// TokenRef wrapper for `Lexer` that additionaly has position.
#[derive(Debug)]
pub struct ItemRef<'t> {
    pub token: TokenRef<'t>,
    pub position: Position,
}

/// Lexer token iterator.
///
/// 'i is iteration lifetime, or "one use of lexer".
/// 't is template lifetime. It will live longer than this iteration.
pub struct Tokens<'i, 't> {
    env: &'i Lexer,
    code: &'t str,
}

impl<'i, 't> Tokens<'i, 't> {
    pub fn new<'ii, 'tt>(lexer: &'ii Lexer, code: &'tt str) -> Tokens<'ii, 'tt> {
        Tokens {
            env: lexer,
            code: code,
        }
    }
}

impl<'i, 't> Iterator for Tokens<'i, 't> {
    type Item = Result<ItemRef<'t>, ()>;

    fn next(&mut self) -> Option<Result<ItemRef<'t>, ()>> {
        return None;
    }
}
